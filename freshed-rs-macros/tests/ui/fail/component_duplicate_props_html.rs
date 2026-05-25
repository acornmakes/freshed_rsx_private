use freshed_rs_macros::html;

#[derive(Default)]
pub struct ButtonProps {
    pub label: &'static str,
}

#[allow(non_snake_case)]
pub fn Button(_props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let _ = html!(<Button label="A" label="B"></Button>);
}
