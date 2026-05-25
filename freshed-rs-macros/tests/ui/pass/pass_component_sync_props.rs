use freshed_rs_macros::html;

pub struct BadgeProps {
    pub tone: &'static str,
    pub children: String,
}

#[allow(non_snake_case)]
pub fn Badge(props: BadgeProps) -> String {
    format!("<Badge tone=\"{}\">{}</Badge>", props.tone, props.children)
}

fn main() {
    let tone = "success";
    let _out = html!(<Badge {tone}>{"ok"}</Badge>);
}
