use freshed_rs_macros::{component, html_async};

#[derive(Default)]
pub struct ButtonProps {}

#[component]
pub fn button(_props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let a = 1;
    let b = 2;
    let _ = html_async!(<Button {a + b}></Button>);
}
