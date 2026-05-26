use freshed_rs_macros::html_ctx;

fn main() {
    let ctx = 1;
    let mut out = String::new();
    let _ = html_ctx!(ctx, &mut out, <div>oops</div>);
}
