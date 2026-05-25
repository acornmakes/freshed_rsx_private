use freshed_rs_macros::html_ide;

pub mod docs {
    pub fn element() {}
}

pub struct HeroProps {
    pub children: String,
}
#[allow(non_snake_case)]
fn Hero(props: HeroProps) -> String {
    format!("<Hero>{}</Hero>", props.children)
}

fn main() {
    let _out = html_ide!(<Hero>{"Headline"}</Hero>);
}
