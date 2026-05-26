use freshed_rs_macros::{component, html};
use freshed_rs_runtime::RenderResult;

#[derive(Default)]
pub struct CardProps {
    pub label: &'static str,
}

#[component]
pub fn card(out: &mut impl ::core::fmt::Write, props: CardProps) -> RenderResult {
    html!(out, <div>{props.label}</div>)
}

fn main() {
    let mut out = String::new();
    let _ = html!(&mut out, <Card label="A" label="B" />);
}
