use freshed_rs_macros::{component, html_async_in};

#[derive(Clone, Copy)]
struct Ctx {
    user: &'static str,
}

pub struct BannerProps {
    pub children: String,
}
#[component]
fn banner(ctx: Ctx, props: BannerProps) -> String {
    format!("<Banner user=\"{}\">{}</Banner>", ctx.user, props.children)
}

fn main() {
    let ctx = Ctx { user: "ava" };
    let _future = html_async_in!(ctx, <Banner>{"welcome"}</Banner>);
}
