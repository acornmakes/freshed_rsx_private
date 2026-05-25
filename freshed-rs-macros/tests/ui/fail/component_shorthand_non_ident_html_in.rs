use freshed_rs_macros::html_ctx;

#[derive(Default)]
pub struct ButtonProps {}

#[allow(non_snake_case)]
pub fn Button(_ctx: i32, _props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let ctx = 1;
    let a = 1;
    let b = 2;
    let _ = html_ctx!(ctx, <Button {a + b}></Button>);
}
