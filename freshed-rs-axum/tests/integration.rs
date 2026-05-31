use axum::{
    body::to_bytes,
    http::{StatusCode, header},
};
use freshed_rs_axum::{render_page, render_result_with, render_stream};
use freshed_rs_runtime::{HtmlFragment, RenderError};

#[tokio::test]
async fn render_page_sets_html_status_header_and_body() {
    let response = render_page(HtmlFragment::from_raw("<p>ok</p>"));

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body should be readable");
    assert_eq!(body, "<p>ok</p>");
}

#[tokio::test]
async fn render_result_with_uses_custom_error_mapper() {
    let response =
        render_result_with::<HtmlFragment, _>(Err(RenderError::Fmt(std::fmt::Error)), |_| {
            (StatusCode::BAD_GATEWAY, "custom error".to_string())
        });

    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body should be readable");
    assert_eq!(body, "custom error");
}

#[tokio::test]
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

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("stream body should be readable");
    assert_eq!(body, "<head><title>x</title></head><body>streamed</body>");
}
