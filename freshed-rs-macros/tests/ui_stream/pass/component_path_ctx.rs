use freshed_rs_macros::html_ctx;
use freshed_rs_runtime::RenderResult;

#[derive(Clone, Copy)]
struct Ctx {
    tenant: &'static str,
}

mod ui {
    use super::{Ctx, RenderResult};
    use freshed_rs_macros::{component, html_ctx, with_children};

    #[with_children]
    #[derive(Default)]
    pub struct PanelProps {
        pub title: &'static str,
    }

    #[component]
    pub fn panel(out: &mut impl ::core::fmt::Write, ctx: Ctx, props: PanelProps) -> RenderResult {
        html_ctx!(
            out,
            ctx,
            <section data-tenant={ctx.tenant} data-title={props.title}>{props.children}</section>
        )
    }
}

fn main() {
    let mut out = String::new();
    let ctx = Ctx { tenant: "acme" };
    html_ctx!(&mut out, ctx, <ui::Panel title="A">{"body"}</ui::Panel>)
        .expect("render should succeed");
}
