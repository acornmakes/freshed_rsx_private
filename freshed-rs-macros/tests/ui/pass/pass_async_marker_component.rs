use freshed_rs_macros::html_async;

pub struct CardProps {
    pub children: String,
}
#[allow(non_snake_case)]
pub async fn Card(props: CardProps) -> String {
    let () = async {}.await;
    format!("<Card>{}</Card>", props.children)
}

fn main() {
    let _future = html_async!(<Card async>{"hello"}</Card>);
}
