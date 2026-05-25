use freshed_rs_macros::{component, html};

pub struct WrapperProps {
    pub children: String,
}
#[component]
fn wrapper(props: WrapperProps) -> String {
    format!("<Wrapper>{}</Wrapper>", props.children)
}

fn main() {
    let _out = html!(<Wrapper />);
}
