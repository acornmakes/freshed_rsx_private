use freshed_rs_macros::html_in;

#[derive(Clone, Copy)]
struct Ctx {
    request_id: &'static str,
}

pub struct ShellProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn Shell(ctx: Ctx, props: ShellProps) -> String {
    format!("<Shell req=\"{}\">{}</Shell>", ctx.request_id, props.children)
}

pub struct TileProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn Tile(ctx: Ctx, props: TileProps) -> String {
    format!("<Tile req=\"{}\">{}</Tile>", ctx.request_id, props.children)
}

fn main() {
    let ctx = Ctx { request_id: "r-1" };
    let _out = html_in!(ctx, <Shell><Tile>{"A"}</Tile><Tile>{"B"}</Tile></Shell>);
}
