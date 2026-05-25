use freshed_rs_macros::{component, html_async_ctx};

#[derive(Default)]
pub struct PanelProps {
    pub children: &'static str,
}

#[component]
pub fn panel(_ctx: i32, _props: PanelProps) -> String {
    String::new()
}

fn main() {
    let ctx = 1;
    let text = "provided";
    let _ = html_async_ctx!(ctx, <Panel children={text}>{"body"}</Panel>);
}
