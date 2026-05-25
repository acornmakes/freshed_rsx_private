use freshed_rs_macros::{component, html_in};

#[derive(Clone, Copy)]
struct Ctx {
    request_id: &'static str,
}

#[derive(Default)]
pub struct ShellProps {
    pub children: String,
}
#[component]
fn shell(ctx: Ctx, props: ShellProps) -> String {
    format!("<Shell req=\"{}\">{}</Shell>", ctx.request_id, props.children)
}

#[derive(Default)]
pub struct TileProps {
    pub children: String,
}
#[component]
fn tile(ctx: Ctx, props: TileProps) -> String {
    format!("<Tile req=\"{}\">{}</Tile>", ctx.request_id, props.children)
}

fn main() {
    let ctx = Ctx { request_id: "r-1" };
    let _out = html_in!(ctx, <Shell><Tile>{"A"}</Tile><Tile>{"B"}</Tile></Shell>);
}
