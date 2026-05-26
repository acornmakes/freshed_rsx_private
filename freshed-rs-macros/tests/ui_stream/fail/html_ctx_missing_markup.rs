use freshed_rs_macros::html_ctx;

fn main() {
    let mut out = String::new();
    let ctx = 1;
    let _ = html_ctx!(&mut out, ctx,);
}
