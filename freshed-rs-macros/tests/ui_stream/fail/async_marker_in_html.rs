use freshed_rs_macros::{component, html, with_children};
use freshed_rs_runtime::RenderResult;

#[with_children]
#[derive(Default)]
pub struct UserCardProps {}

#[component]
pub async fn user_card(out: &mut impl ::core::fmt::Write, props: UserCardProps) -> RenderResult {
    let () = async {}.await;
    html!(out, <div>{props.children}</div>)
}

fn main() {
    let mut out = String::new();
    let _ = html!(&mut out, <UserCard async>{"x"}</UserCard>);
}
