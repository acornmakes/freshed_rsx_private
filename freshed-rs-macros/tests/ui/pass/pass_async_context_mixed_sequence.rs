use freshed_rs_macros::html_async_in;

#[derive(Clone, Copy)]
struct Ctx {
    tenant: &'static str,
}

pub struct SyncBadgeProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn SyncBadge(ctx: Ctx, props: SyncBadgeProps) -> String {
    format!("<SyncBadge tenant=\"{}\">{}</SyncBadge>", ctx.tenant, props.children)
}

pub struct AsyncBadgeProps {
    pub children: String,
}
#[allow(non_snake_case)]
async fn AsyncBadge(ctx: Ctx, props: AsyncBadgeProps) -> String {
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
