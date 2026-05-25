# Functional Design: JSX-Like Stateless Components for `html!`

## 1. Objective

Add a JSX-like component model to `html!` with these properties:

- Component tags are resolved at compile time.
- Components are declared as regular Rust functions (stateless only).
- Properties can be passed with HTML-like syntax.
- Child nodes are captured and passed to the component.
- Nested components are supported.
- Errors are meaningful and point to call-site spans.
- System is SSR-only: no VDOM, no hydration, no live state tracking.
- Request-scoped state (db/session/user/request metadata) is passed consistently to all components.

This design is implementation-ready and aligned with the existing `freshed-rs-macros` architecture.

## 2. Scope

In scope:

- Functional components only.
- Compile-time transform from markup into Rust expressions.
- Props mapping from tag attributes to typed Rust values.
- Children rendering support.
- Context/state threading through nested components.
- Diagnostics for unsupported/invalid component syntax.

Out of scope (for this iteration):

- Stateful components, hooks, lifecycle APIs.
- Any VDOM or DOM diffing layer (now or future in this design).
- Hydration and client-side runtime reconciliation.
- Live state tracking/subscriptions/signals.
- Event systems.
- Spread props (`{...props}`) support.

Explicit non-goal:

- This design never introduces a VDOM. Rendering is direct string generation for SSR.

## 3. User-Facing API

## 3.0 Request Context (State) Model

State is not component-local. State is request-scoped and is threaded explicitly through render calls.

Design contract:

- A single context value (`ctx`) is supplied at macro entry.
- Macro expansion passes `ctx` to every component call automatically.
- Components are pure functions of `(ctx, props)` with no hidden global mutable state.

Example context shape (application-defined):

```rust
pub struct RenderContext<'a> {
    pub db: &'a DbPool,
    pub session: &'a Session,
    pub user: Option<&'a User>,
    pub request_id: &'a str,
}
```

Macro entry forms:

```rust
html_in!(ctx, <Page title="Home" />)
html_async_in!(ctx, <UserCard async user_id={id} />).await
```

`html!` and `html_async!` remain available for context-free trees.

## 3.1 Component Declaration Contract

A sync component is a function with this contract:

```rust
pub fn ComponentName(ctx: &RenderContext<'_>, props: ComponentNameProps) -> impl ::core::fmt::Display
```

An async component is a function with this contract:

```rust
pub async fn ComponentName(ctx: &RenderContext<'_>, props: ComponentNameProps) -> impl ::core::fmt::Display
```

Accepted return types:

- `String`
- `&str`
- Any type implementing `Display`

Required props type convention:

- For `ComponentName`, macro infers props type as `ComponentNameProps` in the same module/path.
- Props type should be a Rust struct.
- If component accepts children, props struct includes field `children`.
- Optional props are declared as `Option<T>` and are omitted at call-sites when desired.

Optional-prop contract:

- Props structs should implement `Default`.
- Omitted optional fields are filled by `..Default::default()` (typically `None` for `Option<T>`).
- Required fields must be supplied by the caller, or Rust emits a missing-field compile error.

Example:

```rust
pub struct ButtonProps {
    pub label: String,
    pub class: String,
    pub children: String,
}

pub fn Button(ctx: &RenderContext<'_>, props: ButtonProps) -> String {
    let is_signed_in = ctx.user.is_some();
    html!(
        <button class={props.class}>
            {props.label}
            {if is_signed_in { "" } else { " (guest)" }}
            {props.children}
        </button>
    )
}
```

## 3.2 Invocation Syntax

Supported:

```rust
html!(<Button label="Save" class="primary" />)

html!(
  <Card title="Welcome">
    <Button label="OK" class="cta" />
    <p>Child html</p>
  </Card>
)

html!(<ui::Button label={caption} class="primary" />)
```

Shorthand prop syntax (JSX-like convenience):

```rust
html!(<Button {label} {class} />)
```

Rules:

- `key="literal"` becomes string literal expression.
- `key={expr}` becomes expression.
- `{ident}` becomes `ident={ident}`.

## 3.3 Tag Classification (Intrinsic vs Component)

A tag is a component when any of the following is true:

- First path segment starts with uppercase ASCII letter (`Button`).
- Tag uses a Rust path separator (`ui::Button`, `crate::ui::Button`).

Otherwise, tag is intrinsic HTML/custom-element behavior (existing behavior).

Examples:

- `<div>` -> intrinsic
- `<my-widget>` -> intrinsic custom element
- `<Button>` -> component
- `<crate::ui::Button>` -> component

## 3.4 Rendering Modes

Two compile-time macros are defined:

- `html!` for synchronous rendering.
- `html_async!` for asynchronous SSR rendering.
- `html_in!` for synchronous rendering with context injection.
- `html_async_in!` for asynchronous SSR rendering with context injection.

Mode rules:

- `html!` supports intrinsic tags and sync component functions.
- `html_async!` supports intrinsic tags, sync components, and async components.
- `html_in!` supports intrinsic tags and sync components that take `ctx`.
- `html_async_in!` supports intrinsic tags, sync components, and async components that take `ctx`.
- `html_async!` expands to an async expression and must be awaited at call site.
- `html_async_in!` expands to an async expression and must be awaited at call site.

Example:

```rust
let page = html_async!(<UserCard async user_id={id} />).await;
let page = html_async_in!(ctx, <UserCard async user_id={id} />).await;
```

## 4. Compile-Time Expansion Model

## 4.1 High-Level Transform

Current macro emits a `format!` with `static_format` + `values`.

Extend this by compiling each node to one of:

- Inline static fragment
- Dynamic expression fragment

For component nodes, emit a dynamic expression that calls the Rust function and returns displayable output.

Async mode extension:

- `html_async!` compiles the same node tree but generates an async block.
- `html_in!` and `html_async_in!` additionally thread a `ctx` expression through each component call.
- Component calls in async mode are awaited when component is marked async by syntax (see below).
- Intrinsic nodes are still rendered directly into strings (no runtime DOM model).

Component elements become `"{}"` placeholders in parent format output, with value expression equal to component call expression.

Async component invocation syntax:

```rust
html_async!(<UserCard async user_id={id} />)
```

Rule:

- The `async` marker on a component tag means the generated call uses `.await`.
- Without the marker, generated call is synchronous.
- In `html!`, using `async` marker is a hard macro error.

## 4.2 Component Call Expansion

Given:

```rust
html!(
  <Card title="T">
    <Button label={x} />
    <span>ok</span>
  </Card>
)
```

Expansion sketch:

```rust
{
    let __children_0 = format!("{}{}",
        Button(ctx, ButtonProps {
            label: x,
            children: String::new(),
            ..::core::default::Default::default()
        }),
        "<span>ok</span>"
    );

    format!("{}",
        Card(ctx, CardProps {
            title: "T".into(),
            children: __children_0,
            ..::core::default::Default::default()
        })
    )
}
```

Notes:

- Exact helper temp names (`__children_n`) are macro-generated and hygienic.
- `..Default::default()` is used to support omitted optional props.
- Missing required fields produce normal Rust struct-literal compile errors.
- Unknown prop names produce Rust field errors with exact spans.

Async expansion sketch:

```rust
{
    async move {
        let __children_0 = format!("{}", "<span>ok</span>");
        format!("{}",
            UserCard(ctx, UserCardProps {
                user_id: id,
                children: __children_0,
                ..::core::default::Default::default()
            }).await
        )
    }
}
```

## 4.3 Props Type Inference

For component path `P::ComponentName`, inferred props type path is `P::ComponentNameProps`.

Examples:

- `Button` -> `ButtonProps`
- `ui::Button` -> `ui::ButtonProps`
- `crate::ui::Button` -> `crate::ui::ButtonProps`

Construction template:

```rust
ComponentPath(ctx, PropsPath {
    key1: value1,
    key2: value2,
    children: rendered_children,
    ..::core::default::Default::default()
})
```

If `children` prop is already supplied explicitly and child nodes are also present, emit macro diagnostic error.

If no child nodes are present and no explicit `children` prop exists, emit `children: String::new()`.

Optional-prop semantics:

- If a prop is omitted, macro does not synthesize an explicit field assignment.
- `..Default::default()` provides values for omitted fields.
- For `Option<T>` optional props, this yields `None` unless overridden by custom `Default`.

## 4.4 Child Rendering Contract

Child nodes of a component are rendered by the same compiler pass into a `String` expression:

- Static text/html collapsed into static format string.
- Dynamic blocks/components become format placeholders.
- Resulting expression is assigned to the generated `children` prop.

This preserves nested component behavior naturally.

## 5. Diagnostics Design

Use `proc_macro2_diagnostics::Diagnostic::spanned` for macro-owned errors/warnings.

## 5.1 Hard Errors

1. Duplicate prop name on component

- Example: `<Button label="A" label="B" />`
- Message: `duplicate property 'label' on component 'Button'`
- Span: second occurrence key span.

2. Mixed explicit children prop and body children

- Example: `<Card children={x}><p>y</p></Card>`
- Message: `children provided both as prop and as child nodes`
- Span: `children` attribute key span.

3. Unsupported bare block in component attrs (non-ident shorthand)

- Example: `<Button {a + b} />`
- Message: `component shorthand prop must be an identifier, e.g. {value}`
- Span: block span.

4. Non self-closing component used with intrinsic empty-element behavior

- Not applicable to components; do not apply intrinsic empty element warnings to component tags.

5. Async marker used in sync mode

- Example: `html!(<UserCard async user_id={id} />)`
- Message: `async component call requires html_async!`
- Span: `async` marker span.

6. Context macro invoked without context argument

- Example: `html_in!(<Page />)`
- Message: `html_in! requires a context expression as first argument`
- Span: macro input start.

7. Context-aware component used in context-free macro

- Example: `html!(<Page />)` where `Page` signature is `(ctx, props)`
- Message: delegated Rust arity error (`this function takes 2 arguments but 1 argument was supplied`).
- Span: component call site.

## 5.2 Rust-Delegated Errors (Desired)

Deliberately rely on Rust type checker for:

- Unknown component symbol (`cannot find function`).
- Missing required props (missing struct fields).
- Unknown prop names (unknown field).
- Wrong prop types.
- Return type not displayable.
- Calling sync component with `.await` due to incorrect async marker usage.
- Incorrect context type passed into context-aware component calls.

These are already meaningful and should preserve call-site spans due to generated struct literal fields using source spans.

## 6. Macro Architecture Changes

## 6.1 Internal IR Refactor

Introduce internal representation:

```rust
enum Fragment {
    Static(String),
    Expr(proc_macro2::TokenStream),
}

struct Compiled {
    format: String,
    values: Vec<proc_macro2::TokenStream>,
    diagnostics: Vec<proc_macro2::TokenStream>,
    collected_elements: Vec<NodeName>,
}
```

`Compiled` can be built from fragments, preserving existing `format!` strategy.

## 6.2 Visitor Logic Changes

In `visit_element`:

1. Classify tag as component vs intrinsic.
2. Intrinsic path keeps existing behavior.
3. Component path:
   - Parse attrs into props map.
   - Compile child nodes to expression `children_expr`.
   - Build component call expression.
   - Append `"{}"` to `static_format` and push call expression to `values`.

Do not add component names to intrinsic semantic element collection.

## 6.3 Attribute Parsing Rules for Components

Supported component attributes:

- `key="literal"`
- `key={expr}`
- `{ident}` shorthand

Rejected for now:

- Bare block not simple identifier
- Attribute blocks intended for raw HTML attribute insertion
- Spread props

## 6.4 New Helper Functions (Macro Crate)

Planned helpers:

- `is_component_tag(name: &NodeName) -> bool`
- `component_paths(name: &NodeName) -> (component_fn_path, props_type_path)`
- `compile_component_call(element: &NodeElement<C>) -> TokenStream`
- `compile_children_to_string(children: &[Node]) -> TokenStream`
- `parse_component_props(attrs: &mut [NodeAttribute]) -> ParsedProps`

## 7. Runtime and Crate Layout Considerations

Current proc-macro crate cannot host reusable runtime symbols for downstream use reliably.

Design decision:

- Keep first iteration runtime-free by emitting plain Rust (`format!`, struct literals, function calls).
- No helper traits required for MVP.
- SSR-only output is plain string generation; no client runtime and no VDOM artifacts are emitted.
- Context type is application-owned; the macro does not prescribe a runtime state container.

Future optional step:

- Add separate non-proc-macro runtime crate (`freshed-rs`) only for shared SSR utilities (escaping helpers, formatting helpers), not for VDOM/state.

## 8. Backward Compatibility

- Existing intrinsic HTML behavior remains unchanged.
- Existing block interpolation behavior remains unchanged.
- `html!` call sites without components are unaffected.
- New async use-cases are additive via `html_async!`.
- Context-aware rendering is additive via `html_in!`/`html_async_in!`.

## 8.1 Modern SSR Concerns

Security and correctness requirements:

- All interpolated text values are HTML-escaped by default.
- A dedicated explicit raw-html wrapper type (for trusted pre-sanitized content) is required for unescaped output.
- Attribute context escaping and text-node escaping must be handled separately.
- Macro-generated code must avoid accidental double-escaping for already-safe wrapper values.

Async and latency requirements:

- Async components are intended for non-blocking IO (database, cache, upstream APIs).
- Async rendering must remain request-scoped and preserve deterministic output order.
- Design permits future incremental streaming (`Write` target) without changing component signatures.

## 9. Implementation Plan

Phase 1:

- Add tag classification and component branch in visitor.
- Emit function call expressions for components.
- Add optional context expression plumbing in macro parser.

Phase 2:

- Implement props parsing and shorthand support.
- Implement children capture and `children` prop injection.

Phase 3:

- Add `html_async!` entrypoint and async rendering pipeline.
- Add `async` component call marker parsing and `.await` expansion.
- Add `html_in!` and `html_async_in!` entrypoints that thread context to all component calls.

Phase 4:

- Add diagnostics for duplicate/mixed children/unsupported shorthand.
- Add diagnostics for invalid async marker usage in `html!`.
- Ensure spans point to source attrs/tag names.

Phase 5:

- Add tests and examples for nested components, type errors, and mixed intrinsic/component markup.

## 10. Test Plan (Must Pass)

Positive tests:

1. Simple component no children.
2. Component with typed expr prop.
3. Component with literal props.
4. Nested components.
5. Component with intrinsic child elements.
6. Path component (`ui::Button`).
7. Shorthand props (`{label}`).
8. Omitted optional prop (`Option<T>`) defaults correctly.
9. Async component render in `html_async!`.
10. Mixed sync + async nested components in `html_async!`.
11. Context propagation to direct child component (`html_in!`).
12. Context propagation through deeply nested components.
13. Async context propagation (`html_async_in!`).
14. Omitted optional prop with context-aware component.

Negative tests:

1. Duplicate prop.
2. `children` passed both ways.
3. Invalid shorthand block (`{a + b}`).
4. Unknown prop field (Rust error).
5. Missing required prop field (Rust error).
6. Unknown component symbol (Rust error).
7. Async marker in `html!` (macro error).
8. Missing context argument in `html_in!`.
9. Wrong context type supplied (Rust type error).

## 11. Example End State

```rust
pub struct CardProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub children: String,
}

pub fn Card(ctx: &RenderContext<'_>, props: CardProps) -> String {
    let user_name = ctx.user.map(|u| u.name.as_str()).unwrap_or("Guest");
    html!(
        <section class="card">
            <h2>{props.title}</h2>
            <small>{user_name}</small>
            <div class="body">{props.children}</div>
        </section>
    )
}

pub struct ButtonProps {
    pub label: String,
    pub children: String,
}

pub fn Button(_ctx: &RenderContext<'_>, props: ButtonProps) -> String {
    html!(<button>{props.label}{props.children}</button>)
}

let title = "Welcome".to_string();
let out = html_in!(ctx,
    <Card {title}>
        <Button label="Save" />
        <span>Now</span>
    </Card>
);

pub struct UserCardProps {
    pub user_id: i64,
    pub children: String,
}

pub async fn UserCard(ctx: &RenderContext<'_>, props: UserCardProps) -> String {
    let user_name = load_user_name(ctx.db, props.user_id).await;
    html!(
        <article>
            <h3>{user_name}</h3>
            {props.children}
        </article>
    )
}

let async_out = html_async_in!(ctx,
    <UserCard async user_id={42}>
        <span>Online</span>
    </UserCard>
)
.await;
```

Expected rendered shape:

```html
<section class="card"><h2>Welcome</h2><div class="body"><button>Save</button><span>Now</span></div></section>
```

Async expected rendered shape:

```html
<article><h3>Alice</h3><span>Online</span></article>
```

## 12. Acceptance Criteria

This design is ready for implementation when:

- Component syntax and classification rules are fixed.
- Expansion template (function call + inferred props struct + children injection) is fixed.
- Context propagation model (entrypoint + automatic threading) is fixed.
- Diagnostic catalog and spans are fixed.
- Test matrix above is implemented.

At that point, implementation can proceed directly in `freshed-rs-macros/src/to_html.rs` with additive tests in `examples` and macro crate tests.
