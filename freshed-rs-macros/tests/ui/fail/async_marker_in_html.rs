use freshed_rs_macros::html;

#[derive(Default)]
pub struct UserCardProps {
    pub children: String,
}

#[allow(non_snake_case)]
pub fn UserCard(props: UserCardProps) -> String {
    format!("<UserCard>{}</UserCard>", props.children)
}

fn main() {
    let _ = html!(<UserCard async>{"x"}</UserCard>);
}
