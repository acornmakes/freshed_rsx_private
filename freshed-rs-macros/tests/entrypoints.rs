use freshed_rs_macros::{
    component, html, html_async, html_async_ctx, html_ctx, html_try, with_children,
};
use freshed_rs_runtime::{CollectHtmlFragmentExt, RawHtml, RenderResult};
use futures::executor::block_on;

#[with_children]
#[derive(Default)]
pub struct CardProps {
    pub title: &'static str,
}

#[component]
pub fn card(out: &mut impl ::core::fmt::Write, props: CardProps) -> RenderResult {
    html!(
        out,
        <section class="card">
            <h2>{props.title}</h2>
            {RawHtml::new(props.children)}
        </section>
    )
}

#[derive(Clone, Copy)]
pub struct Ctx {
    pub tenant: &'static str,
}

#[with_children]
#[derive(Default)]
pub struct BadgeProps {
    pub tone: &'static str,
}

#[component]
pub fn badge(out: &mut impl ::core::fmt::Write, ctx: Ctx, props: BadgeProps) -> RenderResult {
    html!(
        out,
        <span data-tenant={ctx.tenant} data-tone={props.tone}>
            {RawHtml::new(props.children)}
        </span>
    )
}

#[derive(Default)]
pub struct AsyncPanelProps {
    pub label: &'static str,
}

#[component]
pub async fn async_panel(
    out: &mut impl ::core::fmt::Write,
    props: AsyncPanelProps,
) -> RenderResult {
    let () = async {}.await;
    html!(out, <article>{props.label}</article>)
}

#[derive(Default)]
pub struct CtxAsyncPanelProps {
    pub label: &'static str,
}

#[component]
pub async fn ctx_async_panel(
    out: &mut impl ::core::fmt::Write,
    ctx: Ctx,
    props: CtxAsyncPanelProps,
) -> RenderResult {
    let () = async {}.await;
    html!(out, <article data-tenant={ctx.tenant}>{props.label}</article>)
}

#[test]
fn html_writes_intrinsic_markup() {
    let mut out = String::new();
    html!(&mut out, <div>hello</div>).expect("html write should succeed");
    assert_eq!(out, "<div>hello</div>");
}

#[test]
fn html_escapes_dynamic_values() {
    let mut out = String::new();
    let dangerous = "<span>&\"'";
    html!(&mut out, <div title={dangerous}>{dangerous}</div>).expect("html write should succeed");
    assert_eq!(
        out,
        "<div title=\"&lt;span&gt;&amp;&quot;&#39;\">&lt;span&gt;&amp;&quot;&#39;</div>"
    );
}

#[test]
fn html_renders_sync_component() {
    let mut out = String::new();
    html!(&mut out, <Card title="Hello">{"Body"}</Card>).expect("component render should succeed");
    assert_eq!(out, "<section class=\"card\"><h2>Hello</h2>Body</section>");
}

#[test]
fn html_ctx_threads_context_to_components() {
    let mut out = String::new();
    let ctx = Ctx { tenant: "acme" };
    html_ctx!(&mut out, ctx, <Badge tone="success">{"ok"}</Badge>)
        .expect("context render should succeed");
    assert_eq!(
        out,
        "<span data-tenant=\"acme\" data-tone=\"success\">ok</span>"
    );
}

#[test]
fn html_async_supports_intrinsic_markup() {
    let mut out = String::new();
    block_on(html_async!(&mut out, <main>{"async"}</main>)).expect("async render should succeed");
    assert_eq!(out, "<main>async</main>");
}

#[test]
fn html_async_supports_marked_async_components() {
    let mut out = String::new();
    block_on(html_async!(&mut out, <AsyncPanel async label="A" />))
        .expect("async component render should succeed");
    assert_eq!(out, "<article>A</article>");
}

#[test]
fn html_async_ctx_supports_context_and_async_components() {
    let mut out = String::new();
    let ctx = Ctx { tenant: "globex" };
    block_on(html_async_ctx!(
        &mut out,
        ctx,
        <div>
            <Badge tone="info">{"x"}</Badge>
            <CtxAsyncPanel async label="B" />
        </div>
    ))
    .expect("async context render should succeed");

    assert_eq!(
        out,
        "<div><span data-tenant=\"globex\" data-tone=\"info\">x</span><article data-tenant=\"globex\">B</article></div>"
    );
}

#[test]
fn html_fragment_expression_returns_raw_fragment() {
    let mut out = String::new();
    let item = html!(<li>{7}</li>);
    html!(&mut out, <ul>{item}</ul>).expect("fragment interpolation should succeed");
    assert_eq!(out, "<ul><li>7</li></ul>");
}

#[test]
fn collect_html_sequence_supports_iterator_composition() {
    let mut out = String::new();
    let values = vec![0, 1, 2, 3];
    let items = values
        .into_iter()
        .map(|n| html!(<li>{n}</li>))
        .collect_html_sequence();

    html!(&mut out, <ul>{items}</ul>).expect("list rendering should succeed");
    assert_eq!(out, "<ul><li>0</li><li>1</li><li>2</li><li>3</li></ul>");
}

#[test]
fn html_try_returns_result_fragment() {
    let item = html_try!(<li>{42}</li>).expect("fragment render should succeed");

    let mut out = String::new();
    html!(&mut out, <ul>{item}</ul>).expect("fragment interpolation should succeed");
    assert_eq!(out, "<ul><li>42</li></ul>");
}
