use freshed_rs_macros::{component, html_async};

#[derive(Default)]
pub struct PanelProps {
    pub children: &'static str,
}

#[component]
pub fn panel(_props: PanelProps) -> String {
    String::new()
}

fn main() {
    let text = "provided";
    let _ = html_async!(<Panel children={text}>{"body"}</Panel>);
}
