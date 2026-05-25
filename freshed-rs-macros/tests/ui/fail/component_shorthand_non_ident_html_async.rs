use freshed_rs_macros::html_async;

#[derive(Default)]
pub struct ButtonProps {}

#[allow(non_snake_case)]
pub fn Button(_props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let a = 1;
    let b = 2;
    let _ = html_async!(<Button {a + b}></Button>);
}
