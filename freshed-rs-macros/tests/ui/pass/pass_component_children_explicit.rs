use freshed_rs_macros::{component, html};

#[derive(Default)]
pub struct WrapperProps {
    pub title: &'static str,
    pub children: &'static str,
}
#[component]
fn wrapper(props: WrapperProps) -> String {
    format!("<Wrapper title=\"{}\">{}</Wrapper>", props.title, props.children)
}

fn main() {
    let text = "preset";
    let _out = html!(<Wrapper title="T" children={text} />);
}
