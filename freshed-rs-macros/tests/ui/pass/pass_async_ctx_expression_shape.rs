use freshed_rs_macros::{component, html, html_async_in};

#[derive(Clone, Copy)]
struct Ctx {
    id: usize,
}

#[derive(Default)]
pub struct RowProps {
    pub children: String,
}
#[component]
async fn row(ctx: Ctx, props: RowProps) -> String {
    let () = async {}.await;
    // format!("<Row id=\"{}\">{}</Row>", ctx.id, props.children)
    html!(<div><div>{ctx.id}</div>{props.children}</div>)
}

fn main() {
    let _future = html_async_in!(Ctx { id: 9 }, <Row async>{"one"}</Row>);
}
