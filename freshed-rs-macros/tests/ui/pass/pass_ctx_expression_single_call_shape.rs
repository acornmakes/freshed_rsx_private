use freshed_rs_macros::html_in;

#[derive(Clone, Copy)]
struct Ctx {
    id: usize,
}

pub struct RowProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn Row(ctx: Ctx, props: RowProps) -> String {
    format!("<Row id=\"{}\">{}</Row>", ctx.id, props.children)
}

fn make_ctx(seed: usize) -> Ctx {
    Ctx { id: seed + 1 }
}

fn main() {
    let _out = html_in!(make_ctx(4), <Row>{"one"}</Row>);
}
