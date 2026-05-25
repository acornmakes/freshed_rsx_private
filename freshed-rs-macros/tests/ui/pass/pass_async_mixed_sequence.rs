use freshed_rs_macros::{component, html_async};

#[derive(Default)]
pub struct SyncBadgeProps {
    pub children: String,
}
#[component]
fn sync_badge(props: SyncBadgeProps) -> String {
    format!("<SyncBadge>{}</SyncBadge>", props.children)
}

#[derive(Default)]
pub struct AsyncBadgeProps {
    pub children: String,
}
#[component]
async fn async_badge(props: AsyncBadgeProps) -> String {
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
