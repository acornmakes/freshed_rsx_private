use freshed_rs_macros::{component, html_ide};

pub mod docs {
    pub fn element() {}
}

#[derive(Default)]
pub struct HeroProps {
    pub children: String,
}
#[component]
fn hero(props: HeroProps) -> String {
    format!("<Hero>{}</Hero>", props.children)
}

fn main() {
    let _out = html_ide!(<Hero>{"Headline"}</Hero>);
}
