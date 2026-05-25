use freshed_rs_macros::html_in;

pub struct ButtonProps {
    pub label: &'static str,
    pub children: String,
}

#[allow(non_snake_case)]
pub fn Button(_ctx: i32, _props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let ctx = 1;
    let _ = html_in!(ctx, <Button label="A" label="B"></Button>);
}
