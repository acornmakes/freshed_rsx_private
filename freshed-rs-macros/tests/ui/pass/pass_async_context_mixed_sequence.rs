use freshed_rs_macros::{component, html_async_in};

#[derive(Clone, Copy)]
struct Ctx {
    tenant: &'static str,
}

#[derive(Default)]
pub struct SyncBadgeProps {
    pub children: String,
}
#[component]
fn sync_badge(ctx: Ctx, props: SyncBadgeProps) -> String {
    format!("<SyncBadge tenant=\"{}\">{}</SyncBadge>", ctx.tenant, props.children)
}

#[derive(Default)]
pub struct AsyncBadgeProps {
    pub children: String,
}
#[component]
async fn async_badge(ctx: Ctx, props: AsyncBadgeProps) -> String {
    let () = async {}.await;
    format!("<AsyncBadge tenant=\"{}\">{}</AsyncBadge>", ctx.tenant, props.children)
}

fn main() {
    let ctx = Ctx { tenant: "t-1" };
    let _future = html_async_in!(
        ctx,
        <section>
            <SyncBadge>{"A"}</SyncBadge>
            <AsyncBadge async>{"B"}</AsyncBadge>
            <SyncBadge>{"C"}</SyncBadge>
        </section>
    );
}
