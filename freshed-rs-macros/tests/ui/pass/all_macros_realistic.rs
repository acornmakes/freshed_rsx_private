use freshed_rs_macros::{html, html_async, html_async_in, html_ide, html_in};

pub mod docs {
    pub fn element() {}
}

fn main() {
    let _ctx = ("tenant", 9usize);
    let title = "Home";
    let _a = html!(<section><h1>{title}</h1></section>);
    let _b = html_ide!(<div><span>{"x"}</span></div>);
    let _c = html_async!(<article data-id={42}>{"body"}</article>);
    let _d = html_in!(_ctx, <ul><li>{"a"}</li><li>{"b"}</li></ul>);
    let _e = html_async_in!((&_ctx, true), <table><tr><td>{"ok"}</td></tr></table>);
}
