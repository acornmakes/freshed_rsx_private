use freshed_rs_macros::{html, html_to_string, rsx_component, with_children};
use freshed_rs_runtime::{CollectHtmlFragmentExt, RenderResult};
use std::fmt::Write;

fn main() {
    let s = html_to_string!(<div>a</div>).unwrap();
    println!("{}", s);
    let s = html_to_string!(<div><input type="password"></div>).unwrap();
    println!("{}", s);
    let s =
        html_to_string!(<sample-webcomponent random="1" attr2={s}>a</sample-webcomponent>).unwrap();
    println!("{}", s);
    let b = html_to_string!(<Badge tone={"good"} count={Some(20)} />).unwrap();
    println!("{}", b);
    let list = html_to_string!(<Looper count={4} />).unwrap();
    println!("{}", list);
}

#[with_children]
#[derive(Default)]
pub struct BadgeProps {
    pub tone: &'static str,
    pub count: Option<usize>,
}

#[rsx_component]
pub fn Badge(output: &mut impl Write, props: BadgeProps) -> RenderResult {
    let count = props.count.unwrap_or_default();
    let header = html!(<h1>{format!("Heading: {}", props.tone)}</h1>);
    html!(output, <div>{header}<Looper {count} /></div>)
}

#[derive(Default)]
pub struct LooperProps {
    pub count: usize,
}
#[rsx_component]
pub fn Looper(output: &mut impl Write, props: LooperProps) -> RenderResult {
    let items = (0..props.count)
        .map(|n| html!(<li id={format!("li-{:02}",n)}>{n}</li>))
        .collect_html_sequence();
    html!(output, <ul>{
        if props.count == 0 {
            html!(<li>None</li>)
        } else {
            html!(<>{items}</>)
        }
    }</ul>)
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::{Looper, LooperProps};
    use freshed_rs_macros::{html, html_to_string, rsx_component};
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

    #[test]
    fn test_5_collect_html_sequence_loop() {
        let mut out = String::new();
        html!(&mut out, <Looper count={3} />).expect("render should succeed");
        assert_eq!(
            out,
            "<ul><li id=\"li-00\">0</li><li id=\"li-01\">1</li><li id=\"li-02\">2</li></ul>"
        );
    }

    #[derive(Default)]
    pub struct BodyProps;

    #[rsx_component]
    fn Body(output: &mut impl Write, _props: BodyProps) -> RenderResult {
        html!(output, <body><p>hello there</p></body>)
    }

    #[test]
    fn test_doc_outline() {
        let s = html_to_string!(<!DOCTYPE html>
            <html lang="en">
            <head>
                <script type="module" src="https://unpkg.com/@fluentui/web-components@beta"></script>
            </head>
            <Body />
            </html>
        ).unwrap();
        assert_eq!(
            s,
            r#"<!DOCTYPE html><html lang="en"><head><script type="module" src="https://unpkg.com/@fluentui/web-components@beta"></script></head><body><p>hello there</p></body></html>"#
        );
        println!("{}", s);
    }
}
