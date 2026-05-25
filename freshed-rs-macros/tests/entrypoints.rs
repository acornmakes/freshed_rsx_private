use freshed_rs_macros::{html, html_async, html_async_in, html_ide, html_in};

pub mod docs {
    pub fn element() {}
}

#[test]
fn html_renders_basic_intrinsic_markup() {
    let rendered = html!(<div>hello</div>).to_string();
    assert_eq!(rendered, "<div>hello</div>");
}

#[test]
fn html_renders_nested_real_world_page_section() {
    let title = "Dashboard";
    let active = true;
    let rendered = html!(
        <section data-active={active}>
            <h1>{title}</h1>
            <p>{"Welcome back"}</p>
        </section>
    )
    .to_string();
    assert_eq!(
        rendered,
        "<section data-active=\"true\"><h1>Dashboard</h1><p>Welcome back</p></section>"
    );
}

#[test]
fn html_ide_preserves_html_behavior() {
    let rendered = html_ide!(<div>hello</div>).to_string();
    assert_eq!(rendered, "<div>hello</div>");
}

#[test]
fn html_ide_handles_document_like_markup() {
    let rendered = html_ide!(
        <!DOCTYPE html>
        <html>
            <head><title>{"A"}</title></head>
            <body><main>{"B"}</main></body>
        </html>
    )
    .to_string();

    assert_eq!(
        rendered,
        "<!DOCTYPE html><html><head><title>A</title></head><body><main>B</main></body></html>"
    );
}

#[test]
fn html_async_currently_matches_sync_render_shape() {
    let rendered = html_async!(<div>{"async-shape"}</div>).to_string();
    assert_eq!(rendered, "<div>async-shape</div>");
}

#[test]
fn html_async_handles_dynamic_attributes_and_children() {
    let user_id = 42;
    let username = "alice";
    let rendered =
        html_async!(<article data-user-id={user_id}><strong>{username}</strong></article>)
            .to_string();

    assert_eq!(
        rendered,
        "<article data-user-id=\"42\"><strong>alice</strong></article>"
    );
}

#[test]
fn html_in_accepts_context_argument_shape() {
    let _ctx = 7usize;
    let rendered = html_in!(_ctx, <div>{"ctx-shape"}</div>).to_string();
    assert_eq!(rendered, "<div>ctx-shape</div>");
}

#[test]
fn html_in_accepts_complex_context_expression() {
    let _ctx = ("tenant-a", 99usize);
    let rendered = html_in!((&_ctx, 1 + 2), <ul><li>{"one"}</li><li>{"two"}</li></ul>).to_string();
    assert_eq!(rendered, "<ul><li>one</li><li>two</li></ul>");
}

#[test]
fn html_async_in_accepts_context_argument_shape() {
    let _ctx = "request-context";
    let rendered = html_async_in!(_ctx, <div>{"ctx-async-shape"}</div>).to_string();
    assert_eq!(rendered, "<div>ctx-async-shape</div>");
}

#[test]
fn html_async_in_accepts_complex_context_expression() {
    let _ctx = ("session", 11usize);
    let rendered = html_async_in!(Some(&_ctx), <table><tr><td>{"ok"}</td></tr></table>).to_string();
    assert_eq!(rendered, "<table><tr><td>ok</td></tr></table>");
}
