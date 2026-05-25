use freshed_rs_macros::{component, html_ctx};

#[derive(Clone, Copy)]
struct Ctx {
    tenant: &'static str,
}

#[derive(Default)]
pub struct ProfileProps {
    pub children: String,
}
#[component]
fn profile(ctx: Ctx, props: ProfileProps) -> String {
    format!(
        "<Profile tenant=\"{}\">{}</Profile>",
        ctx.tenant, props.children
    )
}

fn main() {
    let ctx = Ctx { tenant: "acme" };
    let _out = html_ctx!(ctx, <Profile>{"ok"}</Profile>);
}
