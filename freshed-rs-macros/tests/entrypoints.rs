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
fn html_ide_preserves_html_behavior() {
    let rendered = html_ide!(<div>hello</div>).to_string();
    assert_eq!(rendered, "<div>hello</div>");
}

#[test]
fn html_async_currently_matches_sync_render_shape() {
    let rendered = html_async!(<div>{"async-shape"}</div>).to_string();
    assert_eq!(rendered, "<div>async-shape</div>");
}

#[test]
fn html_in_accepts_context_argument_shape() {
    let _ctx = 7usize;
    let rendered = html_in!(_ctx, <div>{"ctx-shape"}</div>).to_string();
    assert_eq!(rendered, "<div>ctx-shape</div>");
}

#[test]
fn html_async_in_accepts_context_argument_shape() {
    let _ctx = "request-context";
    let rendered = html_async_in!(_ctx, <div>{"ctx-async-shape"}</div>).to_string();
    assert_eq!(rendered, "<div>ctx-async-shape</div>");
}
