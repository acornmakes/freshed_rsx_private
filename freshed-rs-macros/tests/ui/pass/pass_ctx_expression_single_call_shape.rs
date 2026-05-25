use freshed_rs_macros::{component, html_ctx};

#[derive(Clone, Copy)]
struct Ctx {
    id: usize,
}

#[derive(Default)]
pub struct RowProps {
    pub children: String,
}
#[component]
fn row(ctx: Ctx, props: RowProps) -> String {
    format!("<Row id=\"{}\">{}</Row>", ctx.id, props.children)
}

fn make_ctx(seed: usize) -> Ctx {
    Ctx { id: seed + 1 }
}

fn main() {
    let _out = html_ctx!(make_ctx(4), <Row>{"one"}</Row>);
}
