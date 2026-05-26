use freshed_rs_macros::{html, html_to_string, rsx_component, with_children};
use freshed_rs_runtime::RenderResult;

fn main() {
    println!("Hello, world!");
    let s = html_to_string!(<div>a</div>).unwrap();
    println!("{}", s);
    let b = html_to_string!(<Badge tone={"good"} />).unwrap();
    println!("{}", b);
}

#[with_children]
#[derive(Default)]
pub struct BadgeProps {
    pub tone: &'static str,
    pub count: Option<usize>,
}

#[rsx_component]
pub fn Badge(out: &mut impl ::core::fmt::Write, props: BadgeProps) -> RenderResult {
    let count = props.count.unwrap_or_default();
    html!(out, <div><div>{props.tone}-{count}</div>{props.children}</div>)
}

#[cfg(test)]
mod tests {
    use freshed_rs_macros::{html, rsx_component};
    use freshed_rs_runtime::RenderResult;

    #[test]
    fn test_1() {
        let mut div = String::new();
        html!(&mut div, <div>divided</div>).expect("render should succeed");
        assert_eq!(div, "<div>divided</div>");
    }
    #[test]
    fn test_2() {
        let mut div = String::new();
        html!(&mut div, <div>{123}</div>).expect("render should succeed");
        assert_eq!(div, "<div>123</div>");
    }

    use freshed_rs_macros::{component, with_children};

    #[with_children]
    #[derive(Default)]
    pub struct BadgeProps {
        pub tone: &'static str,
        pub count: Option<usize>,
    }

    #[component]
    pub fn badge(out: &mut impl ::core::fmt::Write, props: BadgeProps) -> RenderResult {
        let count = props.count.unwrap_or_default();
        html!(out, <div><div>{props.tone}-{count}</div>{props.children}</div>)
    }

    #[test]
    fn test_3() {
        let tone = "success";
        let mut out = String::new();
        html!(&mut out, <Badge tone={tone}>{"ok"}</Badge>).expect("render should succeed");
    }

    #[with_children]
    #[derive(Default)]
    pub struct BadgeAdvProps<'a> {
        pub tone: &'a str,
    }

    #[component]
    pub fn badge_adv(out: &mut impl ::core::fmt::Write, props: BadgeAdvProps) -> RenderResult {
        html!(out, <div><div>{props.tone}</div>{props.children}</div>)
    }

    #[test]
    fn test_4() {
        let tone = "success";
        let mut out = String::new();
        html!(&mut out, <BadgeAdv {tone}>{"ok"}</BadgeAdv>).expect("render should succeed");
        assert_eq!(out, "<div><div>success</div>ok</div>")
    }
}
