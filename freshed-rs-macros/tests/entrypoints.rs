use freshed_rs_macros::{component, html, html_async, html_async_in, html_ide, html_in};
use futures::executor::block_on;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod docs {
    pub fn element() {}
}

pub struct CardProps {
    pub children: String,
}

#[component]
pub fn card(props: CardProps) -> String {
    format!("<Card>{}</Card>", props.children)
}

#[allow(non_camel_case_types)]
pub struct user_card_props {
    pub children: String,
}

#[component]
pub fn user_card(props: user_card_props) -> String {
    format!("<UserCard>{}</UserCard>", props.children)
}

pub struct ProfileBadgeProps {
    pub children: String,
}

#[component]
pub fn profile_badge(props: ProfileBadgeProps) -> String {
    format!("<ProfileBadge>{}</ProfileBadge>", props.children)
}

pub struct PanelProps {
    pub title: &'static str,
    pub children: &'static str,
}

#[component]
pub fn panel(props: PanelProps) -> String {
    format!(
        "<Panel title=\"{}\">{}</Panel>",
        props.title, props.children
    )
}

pub struct FancyButtonProps {
    pub label: &'static str,
    pub rank: i32,
    pub tone: &'static str,
    pub children: String,
}

#[component]
pub fn fancy_button(props: FancyButtonProps) -> String {
    format!(
        "<FancyButton label=\"{}\" rank=\"{}\" tone=\"{}\">{}</FancyButton>",
        props.label, props.rank, props.tone, props.children
    )
}

pub struct ActionButtonProps {
    pub count: i32,
    pub kind: &'static str,
    pub action: &'static str,
    pub children: String,
}

#[component]
pub fn action_button(props: ActionButtonProps) -> String {
    format!(
        "<ActionButton count=\"{}\" kind=\"{}\" action=\"{}\">{}</ActionButton>",
        props.count, props.kind, props.action, props.children
    )
}

pub struct MenuItemProps {
    pub label: &'static str,
    pub priority: i32,
    pub glyph: &'static str,
    pub children: String,
}

#[component]
pub fn menu_item(props: MenuItemProps) -> String {
    format!(
        "<MenuItem label=\"{}\" priority=\"{}\" glyph=\"{}\">{}</MenuItem>",
        props.label, props.priority, props.glyph, props.children
    )
}

pub struct AuthBadgeProps {
    pub role: &'static str,
    pub level: i32,
    pub badge: &'static str,
    pub children: String,
}

#[component]
pub fn auth_badge(props: AuthBadgeProps) -> String {
    format!(
        "<AuthBadge role=\"{}\" level=\"{}\" badge=\"{}\">{}</AuthBadge>",
        props.role, props.level, props.badge, props.children
    )
}

pub mod ui {
    pub struct ButtonProps {
        pub data_kind: &'static str,
        pub children: String,
    }

    #[freshed_rs_macros::component]
    pub fn button(props: ButtonProps) -> String {
        format!(
            "<ui::Button data-kind=\"{}\">{}</ui::Button>",
            props.data_kind, props.children
        )
    }
}

pub mod dashboard {
    pub struct PanelProps {
        pub children: String,
    }

    #[freshed_rs_macros::component]
    pub fn panel(props: PanelProps) -> String {
        format!("<dashboard::Panel>{}</dashboard::Panel>", props.children)
    }
}

#[derive(Clone, Copy)]
pub struct RenderCtx {
    pub request_id: &'static str,
    pub tenant: &'static str,
}

pub struct CtxCardProps {
    pub children: String,
}

#[component]
pub fn ctx_card(ctx: RenderCtx, props: CtxCardProps) -> String {
    format!(
        "<CtxCard request-id=\"{}\">{}</CtxCard>",
        ctx.request_id, props.children
    )
}

#[allow(non_camel_case_types)]
pub struct ctx_user_card_props {
    pub children: String,
}

#[component]
pub async fn ctx_user_card(ctx: RenderCtx, props: ctx_user_card_props) -> String {
    let () = async {}.await;
    format!(
        "<CtxUserCard request-id=\"{}\">{}</CtxUserCard>",
        ctx.request_id, props.children
    )
}

pub struct CtxProfileBadgeProps {
    pub children: String,
}

#[component]
pub fn ctx_profile_badge(ctx: RenderCtx, props: CtxProfileBadgeProps) -> String {
    format!(
        "<CtxProfileBadge tenant=\"{}\">{}</CtxProfileBadge>",
        ctx.tenant, props.children
    )
}

pub struct CtxMenuItemProps {
    pub label: &'static str,
    pub priority: i32,
    pub glyph: &'static str,
    pub children: String,
}

#[component]
pub fn ctx_menu_item(ctx: RenderCtx, props: CtxMenuItemProps) -> String {
    format!(
        "<CtxMenuItem request-id=\"{}\" label=\"{}\" priority=\"{}\" glyph=\"{}\">{}</CtxMenuItem>",
        ctx.request_id, props.label, props.priority, props.glyph, props.children
    )
}

pub struct CtxAuthBadgeProps {
    pub role: &'static str,
    pub level: i32,
    pub badge: &'static str,
    pub children: String,
}

#[component]
pub fn ctx_auth_badge(ctx: RenderCtx, props: CtxAuthBadgeProps) -> String {
    format!(
        "<CtxAuthBadge tenant=\"{}\" role=\"{}\" level=\"{}\" badge=\"{}\">{}</CtxAuthBadge>",
        ctx.tenant, props.role, props.level, props.badge, props.children
    )
}

pub mod dashboard_ctx {
    use super::RenderCtx;

    pub struct PanelProps {
        pub children: String,
    }

    #[freshed_rs_macros::component]
    pub fn panel(ctx: RenderCtx, props: PanelProps) -> String {
        format!(
            "<dashboard_ctx::Panel request-id=\"{}\">{}</dashboard_ctx::Panel>",
            ctx.request_id, props.children
        )
    }
}

static CTX_EVAL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
pub struct EvalCtx {
    pub value: &'static str,
}

fn make_eval_ctx() -> EvalCtx {
    CTX_EVAL_COUNT.fetch_add(1, Ordering::SeqCst);
    EvalCtx { value: "ctx-once" }
}

pub struct EvalLeafProps {
    pub children: String,
}

#[component]
pub fn eval_leaf(ctx: EvalCtx, props: EvalLeafProps) -> String {
    format!(
        "<EvalLeaf value=\"{}\">{}</EvalLeaf>",
        ctx.value, props.children
    )
}

pub struct EvalWrapperProps {
    pub children: String,
}

#[component]
pub fn eval_wrapper(ctx: EvalCtx, props: EvalWrapperProps) -> String {
    format!(
        "<EvalWrapper value=\"{}\">{}</EvalWrapper>",
        ctx.value, props.children
    )
}

static RENDER_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

pub struct SeqSyncProps {
    pub label: &'static str,
    pub children: String,
}

#[component]
pub fn seq_sync(props: SeqSyncProps) -> String {
    let order = RENDER_SEQUENCE.fetch_add(1, Ordering::SeqCst);
    format!(
        "<SeqSync label=\"{}\" order=\"{}\">{}</SeqSync>",
        props.label, order, props.children
    )
}

pub struct SeqAsyncProps {
    pub label: &'static str,
    pub children: String,
}

#[component]
pub async fn seq_async(props: SeqAsyncProps) -> String {
    let () = async {}.await;
    let order = RENDER_SEQUENCE.fetch_add(1, Ordering::SeqCst);
    format!(
        "<SeqAsync label=\"{}\" order=\"{}\">{}</SeqAsync>",
        props.label, order, props.children
    )
}

pub struct CtxSeqSyncProps {
    pub label: &'static str,
    pub children: String,
}

#[component]
pub fn ctx_seq_sync(ctx: RenderCtx, props: CtxSeqSyncProps) -> String {
    let order = RENDER_SEQUENCE.fetch_add(1, Ordering::SeqCst);
    format!(
        "<CtxSeqSync tenant=\"{}\" label=\"{}\" order=\"{}\">{}</CtxSeqSync>",
        ctx.tenant, props.label, order, props.children
    )
}

pub struct CtxSeqAsyncProps {
    pub label: &'static str,
    pub children: String,
}

#[component]
pub async fn ctx_seq_async(ctx: RenderCtx, props: CtxSeqAsyncProps) -> String {
    let () = async {}.await;
    let order = RENDER_SEQUENCE.fetch_add(1, Ordering::SeqCst);
    format!(
        "<CtxSeqAsync request-id=\"{}\" label=\"{}\" order=\"{}\">{}</CtxSeqAsync>",
        ctx.request_id, props.label, order, props.children
    )
}

#[test]
fn html_renders_basic_intrinsic_markup() {
    let rendered = html!(<div>hello</div>).to_string();
    assert_eq!(rendered, "<div>hello</div>");
}

#[test]
fn html_renders_nested_real_world_page_section() {
    let title = "Dashboard";
    let active = true;
    let rendered = html!(
        <section data-active={active}>
            <h1>{title}</h1>
            <p>{"Welcome back"}</p>
        </section>
    )
    .to_string();
    assert_eq!(
        rendered,
        "<section data-active=\"true\"><h1>Dashboard</h1><p>Welcome back</p></section>"
    );
}

#[test]
fn html_ide_preserves_html_behavior() {
    let rendered = html_ide!(<div>hello</div>).to_string();
    assert_eq!(rendered, "<div>hello</div>");
}

#[test]
fn html_ide_handles_document_like_markup() {
    let rendered = html_ide!(
        <!DOCTYPE html>
        <html>
            <head><title>{"A"}</title></head>
            <body><main>{"B"}</main></body>
        </html>
    )
    .to_string();

    assert_eq!(
        rendered,
        "<!DOCTYPE html><html><head><title>A</title></head><body><main>B</main></body></html>"
    );
}

#[test]
fn html_async_currently_matches_sync_render_shape() {
    let rendered = block_on(html_async!(<div>{"async-shape"}</div>)).to_string();
    assert_eq!(rendered, "<div>async-shape</div>");
}

#[test]
fn html_async_handles_dynamic_attributes_and_children() {
    let user_id = 42;
    let username = "alice";
    let rendered = block_on(
        html_async!(<article data-user-id={user_id}><strong>{username}</strong></article>),
    )
    .to_string();

    assert_eq!(
        rendered,
        "<article data-user-id=\"42\"><strong>alice</strong></article>"
    );
}

#[test]
fn html_in_accepts_context_argument_shape() {
    let _ctx = 7usize;
    let rendered = html_in!(_ctx, <div>{"ctx-shape"}</div>).to_string();
    assert_eq!(rendered, "<div>ctx-shape</div>");
}

#[test]
fn html_in_accepts_complex_context_expression() {
    let _ctx = ("tenant-a", 99usize);
    let rendered = html_in!((&_ctx, 1 + 2), <ul><li>{"one"}</li><li>{"two"}</li></ul>).to_string();
    assert_eq!(rendered, "<ul><li>one</li><li>two</li></ul>");
}

#[test]
fn html_async_in_accepts_context_argument_shape() {
    let _ctx = "request-context";
    let rendered = block_on(html_async_in!(_ctx, <div>{"ctx-async-shape"}</div>)).to_string();
    assert_eq!(rendered, "<div>ctx-async-shape</div>");
}

#[test]
fn html_async_in_accepts_complex_context_expression() {
    let _ctx = ("session", 11usize);
    let rendered =
        block_on(html_async_in!(Some(&_ctx), <table><tr><td>{"ok"}</td></tr></table>)).to_string();
    assert_eq!(rendered, "<table><tr><td>ok</td></tr></table>");
}

#[test]
fn intrinsic_output_is_consistent_across_no_ctx_macro_modes() {
    let headline = "Overview";
    let count = 3;

    let a = html!(
        <section data-count={count}>
            <h2>{headline}</h2>
            <p>{"Stable"}</p>
        </section>
    )
    .to_string();
    let b = html_ide!(
        <section data-count={count}>
            <h2>{headline}</h2>
            <p>{"Stable"}</p>
        </section>
    )
    .to_string();
    let c = block_on(html_async!(
        <section data-count={count}>
            <h2>{headline}</h2>
            <p>{"Stable"}</p>
        </section>
    ))
    .to_string();

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn intrinsic_output_is_consistent_across_ctx_macro_modes() {
    let _ctx = ("tenant", 4usize);

    let sync_ctx = html_in!(
        &_ctx,
        <nav>
            <a href={"/"}>{"home"}</a>
            <a href={"/about"}>{"about"}</a>
        </nav>
    )
    .to_string();
    let async_ctx = block_on(html_async_in!(
        &_ctx,
        <nav>
            <a href={"/"}>{"home"}</a>
            <a href={"/about"}>{"about"}</a>
        </nav>
    ))
    .to_string();

    assert_eq!(
        sync_ctx,
        "<nav><a href=\"/\">home</a><a href=\"/about\">about</a></nav>"
    );
    assert_eq!(sync_ctx, async_ctx);
}

#[test]
fn html_renders_component_like_uppercase_tag_shape() {
    let rendered = html!(<Card>{"CTA"}</Card>).to_string();
    assert_eq!(rendered, "<Card>CTA</Card>");
}

#[test]
fn html_renders_component_from_snake_case_declaration_macro() {
    let rendered = html!(<UserCard>{"snake"}</UserCard>).to_string();
    assert_eq!(rendered, "<UserCard>snake</UserCard>");
}

#[test]
fn html_ide_renders_component_like_path_tag_shape() {
    let rendered = html_ide!(<ui::Button data_kind={"secondary"}>{"Open"}</ui::Button>).to_string();
    assert_eq!(
        rendered,
        "<ui::Button data-kind=\"secondary\">Open</ui::Button>"
    );
}

#[test]
fn html_async_renders_component_like_uppercase_tag_shape() {
    let rendered = block_on(html_async!(<Card><h3>{"Title"}</h3></Card>)).to_string();
    assert_eq!(rendered, "<Card><h3>Title</h3></Card>");
}

#[test]
fn html_in_renders_component_like_uppercase_tag_shape_with_context() {
    let ctx = RenderCtx {
        request_id: "req-17",
        tenant: "tenant-a",
    };
    let rendered = html_in!(ctx, <CtxProfileBadge>{"ok"}</CtxProfileBadge>).to_string();
    assert_eq!(
        rendered,
        "<CtxProfileBadge tenant=\"tenant-a\">ok</CtxProfileBadge>"
    );
}

#[test]
fn html_async_in_renders_component_like_path_tag_shape_with_context() {
    let ctx = RenderCtx {
        request_id: "req-22",
        tenant: "tenant-a",
    };
    let rendered = block_on(
        html_async_in!(ctx, <dashboard_ctx::Panel><span>{"ok"}</span></dashboard_ctx::Panel>),
    )
    .to_string();
    assert_eq!(
        rendered,
        "<dashboard_ctx::Panel request-id=\"req-22\"><span>ok</span></dashboard_ctx::Panel>"
    );
}

#[test]
fn html_async_in_renders_async_component_from_snake_case_declaration_macro() {
    let ctx = RenderCtx {
        request_id: "req-snake-async",
        tenant: "tenant-snake",
    };
    let rendered =
        block_on(html_async_in!(ctx, <CtxUserCard async>{"ok"}</CtxUserCard>)).to_string();
    assert_eq!(
        rendered,
        "<CtxUserCard request-id=\"req-snake-async\">ok</CtxUserCard>"
    );
}

#[test]
fn intrinsic_custom_element_branch_remains_unchanged_across_macro_families() {
    let _ctx = 1usize;

    let a = html!(<my-widget data-ready={true}>{"x"}</my-widget>).to_string();
    let b = html_ide!(<my-widget data-ready={true}>{"x"}</my-widget>).to_string();
    let c = block_on(html_async!(<my-widget data-ready={true}>{"x"}</my-widget>)).to_string();
    let d = html_in!(_ctx, <my-widget data-ready={true}>{"x"}</my-widget>).to_string();
    let e =
        block_on(html_async_in!(_ctx, <my-widget data-ready={true}>{"x"}</my-widget>)).to_string();

    assert_eq!(a, "<my-widget data-ready=\"true\">x</my-widget>");
    assert_eq!(a, b);
    assert_eq!(a, c);
    assert_eq!(a, d);
    assert_eq!(a, e);
}

#[test]
fn html_component_props_support_literal_expr_and_shorthand_shapes() {
    let tone = "Launch";
    let rank = 2_i32;
    let rendered =
        html!(<FancyButton label="Save" rank={rank} {tone}>{"ok"}</FancyButton>).to_string();
    assert_eq!(
        rendered,
        "<FancyButton label=\"Save\" rank=\"2\" tone=\"Launch\">ok</FancyButton>"
    );
}

#[test]
fn html_ide_component_props_support_children_named_property() {
    let body = "Inline";
    let rendered = html_ide!(<Panel children={body} title="Hello" />).to_string();
    assert_eq!(rendered, "<Panel title=\"Hello\">Inline</Panel>");
}

#[test]
fn html_async_component_props_support_literal_expr_and_shorthand_shapes() {
    let action = "Open";
    let count = 3_i32;
    let rendered = block_on(
        html_async!(<ActionButton count={count} kind="primary" {action}>{"run"}</ActionButton>),
    )
    .to_string();
    assert_eq!(
        rendered,
        "<ActionButton count=\"3\" kind=\"primary\" action=\"Open\">run</ActionButton>"
    );
}

#[test]
fn html_in_component_props_support_literal_expr_and_shorthand_shapes() {
    let ctx = RenderCtx {
        request_id: "req-99",
        tenant: "tenant-z",
    };
    let glyph = "Go";
    let priority = 1_i32;
    let rendered =
        html_in!(ctx, <CtxMenuItem label="File" priority={priority} {glyph}>{"x"}</CtxMenuItem>)
            .to_string();
    assert_eq!(
        rendered,
        "<CtxMenuItem request-id=\"req-99\" label=\"File\" priority=\"1\" glyph=\"Go\">x</CtxMenuItem>"
    );
}

#[test]
fn html_async_in_component_props_support_literal_expr_and_shorthand_shapes() {
    let ctx = RenderCtx {
        request_id: "req-100",
        tenant: "tenant-async",
    };
    let badge = "admin";
    let level = 5_i32;
    let rendered = block_on(html_async_in!(
        ctx,
        <CtxAuthBadge role="owner" level={level} {badge}>{"ok"}</CtxAuthBadge>
    ))
    .to_string();
    assert_eq!(
        rendered,
        "<CtxAuthBadge tenant=\"tenant-async\" role=\"owner\" level=\"5\" badge=\"admin\">ok</CtxAuthBadge>"
    );
}

#[test]
fn html_injects_empty_children_when_component_has_no_body() {
    let rendered = html!(<Card></Card>).to_string();
    assert_eq!(rendered, "<Card></Card>");
}

#[test]
fn html_injects_children_markup_when_component_has_body() {
    let rendered = html!(<Card><span>{"nested"}</span></Card>).to_string();
    assert_eq!(rendered, "<Card><span>nested</span></Card>");
}

#[test]
fn html_async_injects_children_markup_when_component_has_body() {
    let rendered = block_on(html_async!(<Card><strong>{"nested"}</strong></Card>)).to_string();
    assert_eq!(rendered, "<Card><strong>nested</strong></Card>");
}

#[test]
fn html_in_and_html_async_in_inject_children_markup_when_component_has_body() {
    let ctx = RenderCtx {
        request_id: "req-3",
        tenant: "tenant-sync",
    };

    let rendered_sync = html_in!(ctx, <CtxCard><em>{"sync"}</em></CtxCard>).to_string();
    let rendered_async =
        block_on(html_async_in!(ctx, <CtxCard><em>{"sync"}</em></CtxCard>)).to_string();

    assert_eq!(
        rendered_sync,
        "<CtxCard request-id=\"req-3\"><em>sync</em></CtxCard>"
    );
    assert_eq!(rendered_async, rendered_sync);
}

#[test]
fn html_ide_injects_children_markup_when_component_has_body() {
    let rendered = html_ide!(<Card><code>{"child"}</code></Card>).to_string();
    assert_eq!(rendered, "<Card><code>child</code></Card>");
}

#[test]
fn component_children_defaulting_is_consistent_across_all_macro_families() {
    let ctx = RenderCtx {
        request_id: "req-default",
        tenant: "tenant-default",
    };

    let a = html!(<Card></Card>).to_string();
    let b = html_ide!(<Card></Card>).to_string();
    let c = block_on(html_async!(<Card></Card>)).to_string();
    let d = html_in!(ctx, <CtxCard></CtxCard>).to_string();
    let e = block_on(html_async_in!(ctx, <CtxCard></CtxCard>)).to_string();

    assert_eq!(a, "<Card></Card>");
    assert_eq!(a, b);
    assert_eq!(a, c);
    assert_eq!(d, "<CtxCard request-id=\"req-default\"></CtxCard>");
    assert_eq!(d, e);
}

#[test]
fn path_component_children_injection_is_consistent_across_macro_families() {
    let ctx = RenderCtx {
        request_id: "req-path",
        tenant: "tenant-path",
    };

    let a = html!(<dashboard::Panel><span>{"x"}</span></dashboard::Panel>).to_string();
    let b =
        block_on(html_async!(<dashboard::Panel><span>{"x"}</span></dashboard::Panel>)).to_string();
    let c =
        html_in!(ctx, <dashboard_ctx::Panel><span>{"x"}</span></dashboard_ctx::Panel>).to_string();
    let d = block_on(
        html_async_in!(ctx, <dashboard_ctx::Panel><span>{"x"}</span></dashboard_ctx::Panel>),
    )
    .to_string();

    assert_eq!(a, "<dashboard::Panel><span>x</span></dashboard::Panel>");
    assert_eq!(a, b);
    assert_eq!(
        c,
        "<dashboard_ctx::Panel request-id=\"req-path\"><span>x</span></dashboard_ctx::Panel>"
    );
    assert_eq!(c, d);
}

#[test]
fn component_with_explicit_children_prop_and_no_body_is_allowed() {
    let text = "provided";
    let rendered = html!(<Panel title="T" children={text} />).to_string();
    assert_eq!(rendered, "<Panel title=\"T\">provided</Panel>");
}

#[test]
fn nested_component_children_are_composed_through_format_fragments() {
    let rendered = html!(
        <Card>
            <dashboard::Panel>
                <span>{"inside"}</span>
            </dashboard::Panel>
        </Card>
    )
    .to_string();

    assert_eq!(
        rendered,
        "<Card><dashboard::Panel><span>inside</span></dashboard::Panel></Card>"
    );
}

#[test]
fn html_in_evaluates_context_expression_once_and_threads_to_nested_components() {
    CTX_EVAL_COUNT.store(0, Ordering::SeqCst);

    let rendered = html_in!(
        make_eval_ctx(),
        <EvalWrapper>
            <EvalLeaf>{"A"}</EvalLeaf>
            <EvalLeaf>{"B"}</EvalLeaf>
        </EvalWrapper>
    )
    .to_string();

    assert_eq!(CTX_EVAL_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(
        rendered,
        "<EvalWrapper value=\"ctx-once\"><EvalLeaf value=\"ctx-once\">A</EvalLeaf><EvalLeaf value=\"ctx-once\">B</EvalLeaf></EvalWrapper>"
    );
}

#[test]
fn html_async_in_evaluates_context_expression_once_and_threads_to_nested_components() {
    CTX_EVAL_COUNT.store(0, Ordering::SeqCst);

    let rendered = block_on(html_async_in!(
        make_eval_ctx(),
        <EvalWrapper>
            <EvalLeaf>{"X"}</EvalLeaf>
            <EvalLeaf>{"Y"}</EvalLeaf>
        </EvalWrapper>
    ))
    .to_string();

    assert_eq!(CTX_EVAL_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(
        rendered,
        "<EvalWrapper value=\"ctx-once\"><EvalLeaf value=\"ctx-once\">X</EvalLeaf><EvalLeaf value=\"ctx-once\">Y</EvalLeaf></EvalWrapper>"
    );
}

#[test]
fn html_async_awaits_marked_async_components_and_preserves_render_order() {
    RENDER_SEQUENCE.store(0, Ordering::SeqCst);

    let rendered = block_on(html_async!(
        <section>
            <SeqSync label="first" />
            <SeqAsync async label="second" />
            <SeqSync label="third" />
        </section>
    ))
    .to_string();

    assert_eq!(
        rendered,
        "<section><SeqSync label=\"first\" order=\"0\"></SeqSync><SeqAsync label=\"second\" order=\"1\"></SeqAsync><SeqSync label=\"third\" order=\"2\"></SeqSync></section>"
    );
}

#[test]
fn html_async_in_awaits_marked_async_components_and_threads_context() {
    RENDER_SEQUENCE.store(0, Ordering::SeqCst);
    let ctx = RenderCtx {
        request_id: "req-async-seq",
        tenant: "tenant-async-seq",
    };

    let rendered = block_on(html_async_in!(
        ctx,
        <main>
            <CtxSeqSync label="alpha" />
            <CtxSeqAsync async label="beta" />
            <CtxSeqSync label="gamma" />
        </main>
    ))
    .to_string();

    assert_eq!(
        rendered,
        "<main><CtxSeqSync tenant=\"tenant-async-seq\" label=\"alpha\" order=\"0\"></CtxSeqSync><CtxSeqAsync request-id=\"req-async-seq\" label=\"beta\" order=\"1\"></CtxSeqAsync><CtxSeqSync tenant=\"tenant-async-seq\" label=\"gamma\" order=\"2\"></CtxSeqSync></main>"
    );
}

pub struct BoardGameProps<'a> {
    pub name: &'a str,
    pub children: String,
}

#[component]
fn board_game(game: BoardGameProps) -> String {
    html!(<div>{game.name}</div>).to_string()
}

#[test]
fn test_board_game() {
    // let gp = BoardGameProps {
    //     name: &"Ticket to Ride".to_string(),
    // };
    assert_eq!(
        "<div>Ticket to Ride</div>",
        html!(<BoardGame name={"Ticket to Ride"} />)
    )
}
