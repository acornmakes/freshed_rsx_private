use freshed_rs_macros::{component, html};

pub struct WrapperProps {}
#[component]
fn wrapper(props: WrapperProps) -> String {
    let _ = props;
    "<Wrapper></Wrapper>".to_string()
}

fn main() {
    let _out = html!(<Wrapper />);
}
