use freshed_rs_macros::html;

pub struct CardProps {
    pub children: String,
}
#[allow(non_snake_case)]
pub fn Card(props: CardProps) -> String {
    format!("<Card>{}</Card>", props.children)
}

pub struct ItemProps {
    pub label: &'static str,
    pub children: String,
}
#[allow(non_snake_case)]
pub fn Item(props: ItemProps) -> String {
    format!("<Item label=\"{}\">{}</Item>", props.label, props.children)
}

fn main() {
    let _out = html!(<Card><Item label="A" /> <Item label="B" /></Card>);
}
