use freshed_rs_macros::html_async_in;

#[derive(Clone, Copy)]
struct Ctx {
    request_id: &'static str,
}

pub struct UserCardProps {
    pub children: String,
}
#[allow(non_snake_case)]
async fn UserCard(ctx: Ctx, props: UserCardProps) -> String {
    let () = async {}.await;
    format!("<UserCard req=\"{}\">{}</UserCard>", ctx.request_id, props.children)
}

fn main() {
    let ctx = Ctx { request_id: "r-async" };
    let _future = html_async_in!(ctx, <UserCard async>{"ok"}</UserCard>);
}
