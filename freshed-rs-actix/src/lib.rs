use std::{convert::Infallible, fmt, pin::Pin};

use actix_web::HttpMessage;
use actix_web::{
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    http::{StatusCode, header},
};
use freshed_rs_runtime::{HtmlFragment, RenderError, RenderResult};
use futures_util::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

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

impl FromRequest for RenderContext {
    type Error = Error;
    type Future = Pin<Box<dyn futures_util::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let metadata = req
            .extensions()
            .get::<RequestMetadata>()
            .cloned()
            .unwrap_or_else(|| request_metadata_from_request(req));

        async move { Ok(Self { metadata }) }.boxed_local()
    }
}

pub fn default_error_mapper(_error: &RenderError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "<h1>500</h1><p>Rendering failed</p>".to_string(),
    )
}

pub struct HtmlFragmentResponse(pub HtmlFragment);

impl Responder for HtmlFragmentResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        render_page(self.0)
    }
}

pub fn render_page<T>(fragment: T) -> HttpResponse
where
    T: Into<HtmlFragment>,
{
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(fragment.into().into_inner())
}

pub fn render_partial<T>(fragment: T) -> HttpResponse
where
    T: Into<HtmlFragment>,
{
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(fragment.into().into_inner())
}

pub fn render_result<T>(result: Result<T, RenderError>) -> HttpResponse
where
    T: Into<HtmlFragment>,
{
    render_result_with(result, default_error_mapper)
}

pub fn render_result_with<T, F>(result: Result<T, RenderError>, mapper: F) -> HttpResponse
where
    T: Into<HtmlFragment>,
    F: FnOnce(&RenderError) -> (StatusCode, String),
{
    match result {
        Ok(fragment) => render_page(fragment),
        Err(error) => {
            let (status, body) = mapper(&error);
            HttpResponse::build(status)
                .content_type(HTML_CONTENT_TYPE)
                .body(body)
        }
    }
}

pub struct HtmlStreamWriter {
    sender: mpsc::UnboundedSender<Result<actix_web::web::Bytes, Infallible>>,
    pending: String,
    flush_threshold: usize,
}

impl HtmlStreamWriter {
    pub fn new(sender: mpsc::UnboundedSender<Result<actix_web::web::Bytes, Infallible>>) -> Self {
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
            .send(Ok(actix_web::web::Bytes::from(chunk)))
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

pub fn render_stream<F>(render: F) -> HttpResponse
where
    F: FnOnce(&mut HtmlStreamWriter) -> RenderResult + Send + 'static,
{
    let (sender, receiver) = mpsc::unbounded_channel::<Result<actix_web::web::Bytes, Infallible>>();
    actix_web::rt::spawn(async move {
        let mut writer = HtmlStreamWriter::new(sender);
        if let Err(error) = render(&mut writer) {
            let _ = std::fmt::Write::write_str(
                &mut writer,
                &format!("<!-- render error: {:?} -->", error),
            );
        }
        let _ = writer.flush();
    });

    let stream =
        UnboundedReceiverStream::new(receiver).map(|item| item.map_err(|never| match never {}));
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, HTML_CONTENT_TYPE))
        .streaming(stream)
}

pub fn request_metadata_from_request(req: &HttpRequest) -> RequestMetadata {
    let headers = req.headers();
    let request_id = header_value(headers, "x-request-id");
    let auth_user = header_value(headers, "x-auth-user");
    let locale = header_value(headers, header::ACCEPT_LANGUAGE.as_str())
        .and_then(|value| value.split(',').next().map(str::to_string));
    let feature_flags = header_value(headers, "x-feature-flags")
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

fn header_value(headers: &actix_web::http::header::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}
