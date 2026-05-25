use freshed_rs_macros::{component, html};

#[derive(Default)]
pub struct BadgeProps {
    pub tone: &'static str,
    pub children: String,
}

#[component]
pub fn badge(props: BadgeProps) -> String {
    //format!("<Badge tone=\"{}\">{}</Badge>", props.tone, props.children)
    html!(<div><div>{props.tone}</div>{props.children}</div>)
}

fn main() {
    let tone = "success";
    let _out = html!(<Badge {tone}>{"ok"}</Badge>);
}
