use actix_web::{
    body::to_bytes,
    http::{StatusCode, header},
};
use freshed_rs_actix::{
    render_page, render_result_with, render_stream, request_metadata_from_request,
};
use freshed_rs_runtime::{HtmlFragment, RenderError};

#[actix_web::test]
async fn render_page_sets_html_status_header_and_body() {
    let response = render_page(HtmlFragment::from_raw("<p>ok</p>"));

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = to_bytes(response.into_body())
        .await
        .expect("body should be readable");
    assert_eq!(body, "<p>ok</p>");
}

#[actix_web::test]
async fn render_result_with_uses_custom_error_mapper() {
    let response =
        render_result_with::<HtmlFragment, _>(Err(RenderError::Fmt(std::fmt::Error)), |_| {
            (StatusCode::BAD_GATEWAY, "custom error".to_string())
        });

    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);

    let body = to_bytes(response.into_body())
        .await
        .expect("body should be readable");
    assert_eq!(body, "custom error");
}

#[actix_web::test]
async fn render_stream_streams_body_content() {
    let response = render_stream(|out| {
        std::fmt::Write::write_str(out, "<head><title>x</title></head>")
            .map_err(RenderError::from)?;
        out.write_fragment(&HtmlFragment::from_raw("<body>streamed</body>"))?;
        Ok(())
    });

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = to_bytes(response.into_body())
        .await
        .expect("stream body should be readable");
    assert_eq!(body, "<head><title>x</title></head><body>streamed</body>");
}

#[actix_web::test]
async fn request_metadata_helper_reads_headers() {
    let req = actix_web::test::TestRequest::default()
        .insert_header(("x-request-id", "req-1"))
        .insert_header(("x-auth-user", "u42"))
        .insert_header(("accept-language", "en-US,en;q=0.9"))
        .insert_header(("x-feature-flags", "a, b, c"))
        .to_http_request();

    let metadata = request_metadata_from_request(&req);
    assert_eq!(metadata.request_id.as_deref(), Some("req-1"));
    assert_eq!(metadata.auth_user.as_deref(), Some("u42"));
    assert_eq!(metadata.locale.as_deref(), Some("en-US"));
    assert_eq!(metadata.feature_flags, vec!["a", "b", "c"]);
}
