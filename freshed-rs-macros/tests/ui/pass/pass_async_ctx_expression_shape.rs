use freshed_rs_macros::html_async_in;

#[derive(Clone, Copy)]
struct Ctx {
    id: usize,
}

pub struct RowProps {
    pub children: String,
}
#[allow(non_snake_case)]
async fn Row(ctx: Ctx, props: RowProps) -> String {
    let () = async {}.await;
    format!("<Row id=\"{}\">{}</Row>", ctx.id, props.children)
}

fn main() {
    let _future = html_async_in!(Ctx { id: 9 }, <Row async>{"one"}</Row>);
}
