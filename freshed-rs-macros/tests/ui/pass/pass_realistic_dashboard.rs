use freshed_rs_macros::{component, html, html_async, html_async_in, html_in};

#[derive(Clone, Copy)]
struct Ctx {
    tenant: &'static str,
}

#[derive(Default)]
pub struct HeaderProps {
    pub title: &'static str,
    pub children: String,
}
#[component]
fn header(props: HeaderProps) -> String {
    format!("<Header title=\"{}\">{}</Header>", props.title, props.children)
}

#[derive(Default)]
pub struct CtxHeaderProps {
    pub title: &'static str,
    pub children: String,
}
#[component]
fn ctx_header(ctx: Ctx, props: CtxHeaderProps) -> String {
    format!("<CtxHeader tenant=\"{}\" title=\"{}\">{}</CtxHeader>", ctx.tenant, props.title, props.children)
}

#[derive(Default)]
pub struct AsyncPanelProps {
    pub children: String,
}
#[component]
async fn async_panel(props: AsyncPanelProps) -> String {
    let () = async {}.await;
    format!("<AsyncPanel>{}</AsyncPanel>", props.children)
}

#[derive(Default)]
pub struct CtxAsyncPanelProps {
    pub children: String,
}
#[component]
async fn ctx_async_panel(ctx: Ctx, props: CtxAsyncPanelProps) -> String {
    let () = async {}.await;
    format!("<CtxAsyncPanel tenant=\"{}\">{}</CtxAsyncPanel>", ctx.tenant, props.children)
}

fn main() {
    let ctx = Ctx { tenant: "acme" };

    let _sync = html!(<Header title="Overview"><p>{"sync"}</p></Header>);
    let _sync_ctx = html_in!(ctx, <CtxHeader title="Overview"><p>{"ctx"}</p></CtxHeader>);
    let _async = html_async!(<AsyncPanel async><p>{"async"}</p></AsyncPanel>);
    let _async_ctx = html_async_in!(ctx, <CtxAsyncPanel async><p>{"async-ctx"}</p></CtxAsyncPanel>);
}
