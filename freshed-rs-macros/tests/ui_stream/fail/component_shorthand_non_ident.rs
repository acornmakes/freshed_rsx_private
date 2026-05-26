use freshed_rs_macros::{component, html};
use freshed_rs_runtime::RenderResult;

#[derive(Default)]
pub struct ButtonProps;

#[component]
pub fn button(out: &mut impl ::core::fmt::Write, _props: ButtonProps) -> RenderResult {
    html!(out, <button>{"ok"}</button>)
}

fn main() {
    let mut out = String::new();
    let a = 1;
    let b = 2;
    let _ = html!(&mut out, <Button {a + b} />);
}
