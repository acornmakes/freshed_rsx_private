use freshed_rs_macros::{component, html_async};

#[derive(Default)]
pub struct BannerProps {
    pub children: String,
}
#[component]
fn banner(props: BannerProps) -> String {
    format!("<Banner>{}</Banner>", props.children)
}

fn main() {
    let _future = html_async!(<Banner>{"sync fn in async macro"}</Banner>);
}
