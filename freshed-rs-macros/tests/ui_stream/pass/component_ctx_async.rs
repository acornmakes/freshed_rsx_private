use freshed_rs_macros::{component, html_async_ctx, with_children};
use freshed_rs_runtime::RenderResult;

#[derive(Clone, Copy)]
pub struct Ctx {
    tenant: &'static str,
}

#[with_children]
#[derive(Default)]
pub struct BadgeProps {
    pub tone: &'static str,
}

#[component]
pub async fn badge(
    out: &mut impl ::core::fmt::Write,
    ctx: Ctx,
    props: BadgeProps,
) -> RenderResult {
    let () = async {}.await;
    html_async_ctx!(
        out,
        ctx,
        <span data-tenant={ctx.tenant} data-tone={props.tone}>{props.children}</span>
    )
    .await
}

fn main() {
    let mut out = String::new();
    let ctx = Ctx { tenant: "acme" };
    futures::executor::block_on(
        html_async_ctx!(&mut out, ctx, <Badge async tone="success">{"ok"}</Badge>),
    )
    .expect("render should succeed");
}
