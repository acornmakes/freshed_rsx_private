use freshed_rs_macros::html;

mod ui {
    pub struct ButtonProps {
        pub kind: &'static str,
        pub children: String,
    }
    #[freshed_rs_macros::component]
    pub fn button(props: ButtonProps) -> String {
        format!("<ui::Button kind=\"{}\">{}</ui::Button>", props.kind, props.children)
    }
}

fn main() {
    let _out = html!(<ui::Button kind="primary">{"Run"}</ui::Button>);
}
