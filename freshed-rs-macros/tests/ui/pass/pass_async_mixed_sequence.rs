use freshed_rs_macros::html_async;

pub struct SyncBadgeProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn SyncBadge(props: SyncBadgeProps) -> String {
    format!("<SyncBadge>{}</SyncBadge>", props.children)
}

pub struct AsyncBadgeProps {
    pub children: String,
}
#[allow(non_snake_case)]
async fn AsyncBadge(props: AsyncBadgeProps) -> String {
    let () = async {}.await;
    format!("<AsyncBadge>{}</AsyncBadge>", props.children)
}

fn main() {
    let _future = html_async!(
        <section>
            <SyncBadge>{"A"}</SyncBadge>
            <AsyncBadge async>{"B"}</AsyncBadge>
            <SyncBadge>{"C"}</SyncBadge>
        </section>
    );
}
