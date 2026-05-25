use freshed_rs_macros::html;

pub struct WrapperProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn Wrapper(props: WrapperProps) -> String {
    format!("<Wrapper>{}</Wrapper>", props.children)
}

fn main() {
    let _out = html!(<Wrapper />);
}
