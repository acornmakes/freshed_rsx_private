use freshed_rs_macros::html_ctx;

#[derive(Clone, Copy)]
struct Ctx;

#[derive(Default)]
pub struct UserCardProps {
    pub children: String,
}

#[allow(non_snake_case)]
pub fn UserCard(_ctx: Ctx, props: UserCardProps) -> String {
    format!("<UserCard>{}</UserCard>", props.children)
}

fn main() {
    let ctx = Ctx;
    let _ = html_ctx!(ctx, <UserCard async>{"x"}</UserCard>);
}
