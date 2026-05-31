use std::{
    convert::Infallible,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    body::{Body, Bytes},
    extract::FromRequestParts,
    http::{
        HeaderValue, Request, StatusCode,
        header::{self, HeaderName},
        request::Parts,
    },
    response::{IntoResponse, Response},
};
use freshed_rs_runtime::{HtmlFragment, RenderError, RenderResult};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tower::{Layer, Service};

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

pub type ErrorMapper = fn(&RenderError) -> (StatusCode, String);

#[derive(Clone, Debug, Default)]
pub struct RequestMetadata {
    pub request_id: Option<String>,
    pub auth_user: Option<String>,
    pub locale: Option<String>,
    pub feature_flags: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct RenderContext {
    pub metadata: RequestMetadata,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for RenderContext
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(existing) = parts.extensions.get::<RequestMetadata>() {
            return Ok(Self {
                metadata: existing.clone(),
            });
        }

        Ok(Self {
            metadata: request_metadata_from_parts(parts),
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RequestMetadataLayer;

impl<S> Layer<S> for RequestMetadataLayer {
    type Service = RequestMetadataService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestMetadataService { inner }
    }
}

#[derive(Clone, Debug)]
pub struct RequestMetadataService<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for RequestMetadataService<S>
where
    S: Service<Request<ReqBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let metadata = request_metadata_from_headers(req.headers());
        req.extensions_mut().insert(metadata);

        let mut inner = self.inner.clone();
        Box::pin(async move { inner.call(req).await })
    }
}

pub fn default_error_mapper(_error: &RenderError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "<h1>500</h1><p>Rendering failed</p>".to_string(),
    )
}

pub fn render_page<T>(fragment: T) -> Response
where
    T: Into<HtmlFragment>,
{
    response_with_html(
        StatusCode::OK,
        fragment.into().into_inner(),
        HTML_CONTENT_TYPE,
    )
}

pub fn render_partial<T>(fragment: T) -> Response
where
    T: Into<HtmlFragment>,
{
    response_with_html(
        StatusCode::OK,
        fragment.into().into_inner(),
        HTML_CONTENT_TYPE,
    )
}

pub fn render_result<T>(result: Result<T, RenderError>) -> Response
where
    T: Into<HtmlFragment>,
{
    render_result_with(result, default_error_mapper)
}

pub fn render_result_with<T, F>(result: Result<T, RenderError>, mapper: F) -> Response
where
    T: Into<HtmlFragment>,
    F: FnOnce(&RenderError) -> (StatusCode, String),
{
    match result {
        Ok(fragment) => render_page(fragment),
        Err(error) => {
            let (status, body) = mapper(&error);
            response_with_html(status, body, HTML_CONTENT_TYPE)
        }
    }
}

pub struct HtmlStreamWriter {
    sender: mpsc::UnboundedSender<Result<Bytes, Infallible>>,
    pending: String,
    flush_threshold: usize,
}

impl HtmlStreamWriter {
    pub fn new(sender: mpsc::UnboundedSender<Result<Bytes, Infallible>>) -> Self {
        Self {
            sender,
            pending: String::new(),
            flush_threshold: 8 * 1024,
        }
    }

    pub fn set_flush_threshold(&mut self, bytes: usize) {
        self.flush_threshold = bytes.max(1);
    }

    pub fn flush(&mut self) -> Result<(), fmt::Error> {
        if self.pending.is_empty() {
            return Ok(());
        }

        let chunk = std::mem::take(&mut self.pending);
        self.sender
            .send(Ok(Bytes::from(chunk)))
            .map_err(|_| fmt::Error)
    }

    pub fn write_fragment(&mut self, fragment: &HtmlFragment) -> RenderResult {
        fragment.render_to(self)
    }
}

impl fmt::Write for HtmlStreamWriter {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.pending.push_str(s);
        if self.pending.len() >= self.flush_threshold {
            self.flush()?;
        }
        Ok(())
    }
}

impl Drop for HtmlStreamWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

pub fn render_stream<F>(render: F) -> Response
where
    F: FnOnce(&mut HtmlStreamWriter) -> RenderResult + Send + 'static,
{
    let (sender, receiver) = mpsc::unbounded_channel::<Result<Bytes, Infallible>>();
    tokio::spawn(async move {
        let mut writer = HtmlStreamWriter::new(sender);
        if let Err(error) = render(&mut writer) {
            let _ = std::fmt::Write::write_str(
                &mut writer,
                &format!("<!-- render error: {:?} -->", error),
            );
        }
        let _ = writer.flush();
    });

    let stream = UnboundedReceiverStream::new(receiver);
    response_with_stream(StatusCode::OK, stream, HTML_CONTENT_TYPE)
}

pub fn response_with_stream<S>(
    status: StatusCode,
    stream: S,
    content_type: &'static str,
) -> Response
where
    S: tokio_stream::Stream<Item = Result<Bytes, Infallible>> + Send + 'static,
{
    let mut response = Body::from_stream(stream).into_response();
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    response
}

fn response_with_html(status: StatusCode, body: String, content_type: &'static str) -> Response {
    let mut response = body.into_response();
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    response
}

pub fn request_metadata_from_parts(parts: &Parts) -> RequestMetadata {
    request_metadata_from_headers(&parts.headers)
}

pub fn request_metadata_from_headers(headers: &axum::http::HeaderMap) -> RequestMetadata {
    let request_id = header_value(headers, &header::HeaderName::from_static("x-request-id"));
    let auth_user = header_value(headers, &header::HeaderName::from_static("x-auth-user"));
    let locale = header_value(headers, &header::ACCEPT_LANGUAGE)
        .and_then(|value| value.split(',').next().map(str::to_string));
    let feature_flags = header_value(headers, &HeaderName::from_static("x-feature-flags"))
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|flag| !flag.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();

    RequestMetadata {
        request_id,
        auth_user,
        locale,
        feature_flags,
    }
}

fn header_value(headers: &axum::http::HeaderMap, name: &HeaderName) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}
