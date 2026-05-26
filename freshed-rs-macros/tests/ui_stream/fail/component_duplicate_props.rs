use freshed_rs_macros::{component, html};
use freshed_rs_runtime::RenderResult;

#[derive(Default)]
pub struct ButtonProps {
    pub label: &'static str,
}

#[component]
pub fn button(out: &mut impl ::core::fmt::Write, props: ButtonProps) -> RenderResult {
    html!(out, <button>{props.label}</button>)
}

fn main() {
    let mut out = String::new();
    let _ = html!(&mut out, <Button label="A" label="B" />);
}
