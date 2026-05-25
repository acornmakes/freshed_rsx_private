use freshed_rs_macros::html_async;

pub struct BannerProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn Banner(props: BannerProps) -> String {
    format!("<Banner>{}</Banner>", props.children)
}

fn main() {
    let _future = html_async!(<Banner>{"sync fn in async macro"}</Banner>);
}
