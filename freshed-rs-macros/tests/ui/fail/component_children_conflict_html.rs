use freshed_rs_macros::{component, html};

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
    let _ = html!(<Panel children={text}>{"body"}</Panel>);
}
