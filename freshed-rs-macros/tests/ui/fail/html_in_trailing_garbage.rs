use freshed_rs_macros::html_ctx;

fn main() {
    let _ctx = 9;
    let _ = html_ctx!(_ctx, <div>ok</div>,);
}
