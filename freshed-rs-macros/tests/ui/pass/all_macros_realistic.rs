use freshed_rs_macros::{html, html_async, html_async_in, html_ide, html_in};

pub mod docs {
    pub fn element() {}
}

#[derive(Clone, Copy)]
struct RenderCtx {
    tenant: &'static str,
}

pub struct HeaderProps {
    pub title: &'static str,
    pub children: String,
}

#[allow(non_snake_case)]
fn Header(props: HeaderProps) -> String {
    format!("<Header title=\"{}\">{}</Header>", props.title, props.children)
}

pub struct CtxHeaderProps {
    pub title: &'static str,
    pub children: String,
}

#[allow(non_snake_case)]
fn CtxHeader(ctx: RenderCtx, props: CtxHeaderProps) -> String {
    format!(
        "<CtxHeader tenant=\"{}\" title=\"{}\">{}</CtxHeader>",
        ctx.tenant, props.title, props.children
    )
}

pub struct AsyncPanelProps {
    pub children: String,
}

#[allow(non_snake_case)]
async fn AsyncPanel(props: AsyncPanelProps) -> String {
    let () = async {}.await;
    format!("<AsyncPanel>{}</AsyncPanel>", props.children)
}

pub struct CtxAsyncPanelProps {
    pub children: String,
}

#[allow(non_snake_case)]
async fn CtxAsyncPanel(ctx: RenderCtx, props: CtxAsyncPanelProps) -> String {
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
    let _d = html_in!(_ctx, <CtxHeader title="Ctx"><ul><li>{"a"}</li><li>{"b"}</li></ul></CtxHeader>);
    let _e = html_async_in!(_ctx, <CtxAsyncPanel async><table><tr><td>{"ok"}</td></tr></table></CtxAsyncPanel>);
}
