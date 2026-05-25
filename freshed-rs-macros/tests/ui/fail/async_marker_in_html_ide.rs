use freshed_rs_macros::html_ide;

pub mod docs {
    pub fn element() {}
}

pub struct UserCardProps {
    pub children: String,
}

#[allow(non_snake_case)]
pub fn UserCard(props: UserCardProps) -> String {
    format!("<UserCard>{}</UserCard>", props.children)
}

fn main() {
    let _ = html_ide!(<UserCard async>{"x"}</UserCard>);
}
