use freshed_rs_macros::{component, html_async};

pub struct PanelProps {
    pub children: String,
}
#[component]
pub fn panel(props: PanelProps) -> String {
    format!("<Panel>{}</Panel>", props.children)
}

pub struct UserCardProps {
    pub children: String,
}
#[component]
pub async fn user_card(props: UserCardProps) -> String {
    let () = async {}.await;
    format!("<UserCard>{}</UserCard>", props.children)
}

fn main() {
    let _future = html_async!(<Panel><UserCard async>{"u"}</UserCard></Panel>);
}
