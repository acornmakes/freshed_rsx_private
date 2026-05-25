use freshed_rs_macros::html_ide;

pub struct ButtonProps {
    pub label: &'static str,
}

#[allow(non_snake_case)]
pub fn Button(_props: ButtonProps) -> String {
    String::new()
}

pub mod docs {
    pub fn element() {}
}

fn main() {
    let _ = html_ide!(<Button label="A" label="B"></Button>);
}
