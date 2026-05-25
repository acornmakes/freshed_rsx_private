use freshed_rs_macros::html_in;

pub struct PanelProps {
    pub children: &'static str,
}

#[allow(non_snake_case)]
pub fn Panel(_ctx: i32, _props: PanelProps) -> String {
    String::new()
}

fn main() {
    let ctx = 1;
    let text = "provided";
    let _ = html_in!(ctx, <Panel children={text}>{"body"}</Panel>);
}
