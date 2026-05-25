use freshed_rs_macros::{component, html_in};

#[derive(Clone, Copy)]
struct Ctx {
    tenant: &'static str,
}

pub struct ProfileProps {
    pub children: String,
}
#[component]
fn profile(ctx: Ctx, props: ProfileProps) -> String {
    format!("<Profile tenant=\"{}\">{}</Profile>", ctx.tenant, props.children)
}

fn main() {
    let ctx = Ctx { tenant: "acme" };
    let _out = html_in!(ctx, <Profile>{"ok"}</Profile>);
}
