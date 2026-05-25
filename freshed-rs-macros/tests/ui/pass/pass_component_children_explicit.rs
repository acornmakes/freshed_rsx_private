use freshed_rs_macros::html;

pub struct WrapperProps {
    pub title: &'static str,
    pub children: &'static str,
}
#[allow(non_snake_case)]
fn Wrapper(props: WrapperProps) -> String {
    format!("<Wrapper title=\"{}\">{}</Wrapper>", props.title, props.children)
}

fn main() {
    let text = "preset";
    let _out = html!(<Wrapper title="T" children={text} />);
}
