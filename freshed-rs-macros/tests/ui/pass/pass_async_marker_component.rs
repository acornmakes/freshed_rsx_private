use freshed_rs_macros::{component, html_async};

#[derive(Default)]
pub struct CardProps {
    pub children: String,
}
#[component]
pub async fn card(props: CardProps) -> String {
    let () = async {}.await;
    format!("<Card>{}</Card>", props.children)
}

fn main() {
    let _future = html_async!(<Card async>{"hello"}</Card>);
}
