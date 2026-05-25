use freshed_rs_macros::{component, html, html_async, html_async_in, html_ide, html_in};

pub mod docs {
    pub fn element() {}
}

#[derive(Clone, Copy)]
struct RenderCtx {
    tenant: &'static str,
}

#[derive(Default)]
pub struct HeaderProps {
    pub title: &'static str,
    pub children: String,
}

#[component]
fn header(props: HeaderProps) -> String {
    format!(
        "<Header title=\"{}\">{}</Header>",
        props.title, props.children
    )
}

#[derive(Default)]
pub struct CtxHeaderProps {
    pub title: &'static str,
    pub children: String,
}

#[component]
fn ctx_header(ctx: RenderCtx, props: CtxHeaderProps) -> String {
    format!(
        "<CtxHeader tenant=\"{}\" title=\"{}\">{}</CtxHeader>",
        ctx.tenant, props.title, props.children
    )
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
async fn ctx_async_panel(ctx: RenderCtx, props: CtxAsyncPanelProps) -> String {
    let () = async {}.await;
    format!(
        "<CtxAsyncPanel tenant=\"{}\">{}</CtxAsyncPanel>",
        ctx.tenant, props.children
    )
}

fn main() {
    let _ctx = RenderCtx { tenant: "acme" };
    let title = "Home";
    let _a = html!(<Header title={title}><h1>{title}</h1></Header>);
    let _b = html_ide!(<div><span>{"x"}</span></div>);
    let _c = html_async!(<AsyncPanel async><article data-id={42}>{"body"}</article></AsyncPanel>);
    let _d =
        html_in!(_ctx, <CtxHeader title="Ctx"><ul><li>{"a"}</li><li>{"b"}</li></ul></CtxHeader>);
    let _e = html_async_in!(_ctx, <CtxAsyncPanel async><table><tr><td>{"ok"}</td></tr></table></CtxAsyncPanel>);
}
