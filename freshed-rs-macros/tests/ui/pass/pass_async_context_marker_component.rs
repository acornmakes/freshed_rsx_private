use freshed_rs_macros::{component, html_async_in};

#[derive(Clone, Copy)]
struct Ctx {
    request_id: &'static str,
}

pub struct UserCardProps {
    pub children: String,
}
#[component]
async fn user_card(ctx: Ctx, props: UserCardProps) -> String {
    let () = async {}.await;
    format!("<UserCard req=\"{}\">{}</UserCard>", ctx.request_id, props.children)
}

fn main() {
    let ctx = Ctx { request_id: "r-async" };
    let _future = html_async_in!(ctx, <UserCard async>{"ok"}</UserCard>);
}
