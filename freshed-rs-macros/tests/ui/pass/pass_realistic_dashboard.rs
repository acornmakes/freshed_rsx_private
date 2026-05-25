use freshed_rs_macros::{html, html_async, html_async_in, html_in};

#[derive(Clone, Copy)]
struct Ctx {
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
fn CtxHeader(ctx: Ctx, props: CtxHeaderProps) -> String {
    format!("<CtxHeader tenant=\"{}\" title=\"{}\">{}</CtxHeader>", ctx.tenant, props.title, props.children)
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
async fn CtxAsyncPanel(ctx: Ctx, props: CtxAsyncPanelProps) -> String {
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
