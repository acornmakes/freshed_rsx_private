use freshed_rs_macros::html_async;

pub struct PanelProps {
    pub children: &'static str,
}

#[allow(non_snake_case)]
pub fn Panel(_props: PanelProps) -> String {
    String::new()
}

fn main() {
    let text = "provided";
    let _ = html_async!(<Panel children={text}>{"body"}</Panel>);
}
