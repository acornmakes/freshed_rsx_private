use freshed_rs_macros::{html_async_in, html_in};

fn build_ctx() -> (String, usize) {
    ("session-1".to_string(), 10)
}

fn main() {
    let _base = build_ctx();
    let _ = html_in!((&_base, Some(3usize)), <div>{"shape-a"}</div>);
    let _ = html_async_in!((build_ctx(), false, 1 + 2), <div>{"shape-b"}</div>);
}
