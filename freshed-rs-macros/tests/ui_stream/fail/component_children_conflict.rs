use freshed_rs_macros::{component, html, with_children};
use freshed_rs_runtime::RenderResult;

#[with_children]
#[derive(Default)]
pub struct PanelProps {}

#[component]
pub fn panel(out: &mut impl ::core::fmt::Write, props: PanelProps) -> RenderResult {
    html!(out, <section>{props.children}</section>)
}

fn main() {
    let mut out = String::new();
    let text = "provided";
    let _ = html!(&mut out, <Panel children={text}>{"body"}</Panel>);
}
