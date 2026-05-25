use freshed_rs_macros::html_async;

pub struct ButtonProps {
    pub label: &'static str,
    pub children: String,
}

#[allow(non_snake_case)]
pub fn Button(_props: ButtonProps) -> String {
    String::new()
}

fn main() {
    let _ = html_async!(<Button label="A" label="B"></Button>);
}
