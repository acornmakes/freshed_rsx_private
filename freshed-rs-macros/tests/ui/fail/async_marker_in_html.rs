use freshed_rs_macros::{component, html};

#[derive(Default)]
pub struct UserCardProps {
    pub children: String,
}

#[component]
pub fn user_card(props: UserCardProps) -> String {
    format!("<UserCard>{}</UserCard>", props.children)
}

fn main() {
    let _ = html!(<UserCard async>{"x"}</UserCard>);
}
