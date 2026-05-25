use freshed_rs_macros::html_async_ctx;

#[derive(Default)]
pub struct ButtonProps {
    pub label: &'static str,
}

#[allow(non_snake_case)]
pub fn Button(_ctx: i32, _props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let ctx = 1;
    let _ = html_async_ctx!(ctx, <Button label="A" label="B"></Button>);
}
