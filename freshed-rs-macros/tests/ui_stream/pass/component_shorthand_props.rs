use freshed_rs_macros::{component, html, with_children};
use freshed_rs_runtime::RenderResult;

#[with_children]
#[derive(Default)]
pub struct BadgeProps {
    pub tone: &'static str,
    pub level: i32,
}

#[component]
pub fn badge(out: &mut impl ::core::fmt::Write, props: BadgeProps) -> RenderResult {
    html!(
        out,
        <span data-tone={props.tone} data-level={props.level}>{props.children}</span>
    )
}

fn main() {
    let mut out = String::new();
    let tone = "success";
    let level = 2;
    html!(&mut out, <Badge {tone} {level}>{"ok"}</Badge>).expect("render should succeed");
}
