fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use freshed_rs_macros::html;

    #[test]
    fn test_1() {
        let div = html!(
            <div>divided</div>
        )
        .to_string();
        assert_eq!(div, "<div>divided</div>");
    }
    #[test]
    fn test_2() {
        let div = html!(
            <div>{123}</div>
        )
        .to_string();
        assert_eq!(div, "<div>123</div>");
    }

    use freshed_rs_macros::{component, with_children};

    #[with_children]
    #[derive(Default)]
    pub struct BadgeProps {
        pub tone: &'static str,
    }

    #[component]
    pub fn badge(props: BadgeProps) -> String {
        html!(<div><div>{props.tone}</div>{props.children}</div>)
    }

    #[test]
    fn test_3() {
        let tone = "success";
        let _out = html!(<Badge {tone}>{"ok"}</Badge>);
    }

    #[with_children]
    #[derive(Default)]
    pub struct BadgeAdvProps<'a> {
        pub tone: &'a str,
    }

    #[component]
    pub fn badge_adv(props: BadgeAdvProps) -> String {
        html!(<div><div>{props.tone}</div>{props.children}</div>)
    }

    #[test]
    fn test_4() {
        let tone = "success";
        let _out = html!(<BadgeAdv {tone}>{"ok"}</BadgeAdv>);
        assert_eq!(_out, "<div><div>success</div>ok</div>")
    }
}
