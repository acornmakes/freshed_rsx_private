use freshed_rs_macros::html;

pub struct PanelProps {
    pub children: &'static str,
}

#[allow(non_snake_case)]
pub fn Panel(_props: PanelProps) -> String {
    String::new()
}

fn main() {
    let text = "provided";
    let _ = html!(<Panel children={text}>{"body"}</Panel>);
}
