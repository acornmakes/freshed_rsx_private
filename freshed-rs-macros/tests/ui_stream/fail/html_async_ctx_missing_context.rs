use freshed_rs_macros::html_async_ctx;

fn main() {
    let mut out = String::new();
    let _ = html_async_ctx!(&mut out, <div>oops</div>);
}
