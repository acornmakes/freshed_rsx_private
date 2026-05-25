use freshed_rs_macros::html_async_in;

#[derive(Clone, Copy)]
struct Ctx {
    user: &'static str,
}

pub struct BannerProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn Banner(ctx: Ctx, props: BannerProps) -> String {
    format!("<Banner user=\"{}\">{}</Banner>", ctx.user, props.children)
}

fn main() {
    let ctx = Ctx { user: "ava" };
    let _future = html_async_in!(ctx, <Banner>{"welcome"}</Banner>);
}
