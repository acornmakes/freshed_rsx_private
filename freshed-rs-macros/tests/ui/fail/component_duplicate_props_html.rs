use freshed_rs_macros::{component, html};

#[derive(Default)]
pub struct ButtonProps {
    pub label: &'static str,
}

#[component]
pub fn button(_props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let _ = html!(<Button label="A" label="B"></Button>);
}
