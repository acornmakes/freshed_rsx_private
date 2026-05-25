use freshed_rs_macros::html_async;

pub struct PanelProps {
    pub children: String,
}
#[allow(non_snake_case)]
pub fn Panel(props: PanelProps) -> String {
    format!("<Panel>{}</Panel>", props.children)
}

pub struct UserCardProps {
    pub children: String,
}
#[allow(non_snake_case)]
pub async fn UserCard(props: UserCardProps) -> String {
    let () = async {}.await;
    format!("<UserCard>{}</UserCard>", props.children)
}

fn main() {
    let _future = html_async!(<Panel><UserCard async>{"u"}</UserCard></Panel>);
}
