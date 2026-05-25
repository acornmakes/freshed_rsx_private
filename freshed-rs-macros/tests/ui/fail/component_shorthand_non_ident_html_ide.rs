use freshed_rs_macros::html_ide;

pub struct ButtonProps {
    pub children: String,
}

#[allow(non_snake_case)]
pub fn Button(_props: ButtonProps) -> String {
    String::new()
}

pub mod docs {
    pub fn element() {}
}

fn main() {
    let a = 1;
    let b = 2;
    let _ = html_ide!(<Button {a + b}></Button>);
}
