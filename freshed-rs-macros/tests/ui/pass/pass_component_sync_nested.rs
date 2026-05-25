use freshed_rs_macros::{component, html};

pub struct CardProps {
    pub children: String,
}
#[component]
pub fn card(props: CardProps) -> String {
    format!("<Card>{}</Card>", props.children)
}

pub struct ItemProps {
    pub label: &'static str,
}
#[component]
pub fn item(props: ItemProps) -> String {
    format!("<Item label=\"{}\"></Item>", props.label)
}

fn main() {
    let _out = html!(<Card><Item label="A" /> <Item label="B" /></Card>);
}
