# Functional Design: Streamed HTML Writing

## Objective

Move SSR generation from repeated `String` construction toward an idiomatic writer-based pipeline while keeping component call-sites ergonomic and context passing consistent.

## Why Change

Current expansion is mostly `format!` + intermediate `String`s. For large pages and nested components this creates many allocations and copies.

A writer model can:

- reduce temporary allocations
- improve throughput for large responses
- let callers stream directly to buffers/sockets/adapters

## Core Decision

Use one writer passed once at the macro entry point, then thread it through all nested component calls in generated code.

The goal is:

- explicit at the top level
- mostly hidden below that level
- always consistent

## Proposed API Surface

Do NOT Keep existing macros for compatibility. 

- `html!`
- `html_async!`
- `html_ctx!`
- `html_async_ctx!`

Change the above to  writer-oriented variants:

- `html!(writer, <markup...>) -> RenderResult`
- `html_ctx!(writer, ctx, <markup...>) -> RenderResult`
- `html_async!(writer, <markup...>) -> impl Future<Output = RenderResult>`
- `html_async_ctx!(writer, ctx, <markup...>) -> impl Future<Output = RenderResult>`

Where:

- `type RenderResult = Result<(), RenderError>`
- `writer` is `&mut impl std::fmt::Write` in first implementation phase

`std::fmt::Write` is the easiest first step because it works naturally with UTF-8 HTML and avoids immediate async trait complexity.

## Component Contract Changes

### Recommended (stream-native)

Allow component functions that accept writer directly:

```rust
#[component]
pub fn user_card(out: &mut impl std::fmt::Write, ctx: &RenderContext<'_>,  props: UserCardProps) -> RenderResult {
    html_ctx!(out, ctx, <section class="card">{props.children}</section>)
}
```

The macro-generated PascalCase wrapper should preserve writer threading:

```rust
UserCard(out, ctx, props)?;
```

### Compatibility Mode

Do not preserve compatibility as this code has not been released. All tests and example code should be updated. 

## How `html!*` Expansion Would Change

Instead of assembling one `format!`, emit direct writes:

- literals: `out.write_str("<div>")?;`
- dynamic expressions: `write!(out, "{}", expr)?;`
- nested components: `Component(out, ctx, props)?;`

This keeps writer plumbing in macro expansion rather than user code.

## Children Semantics

Current `#[with_children]` injects `children: String`.

For streaming there are 2 options:

1. Transitional option (recommended first):
- keep `children: String`
- only allocate a temporary `String` when a component actually consumes children
- everything else streams directly

2. Full streaming children (later):
- replace `children: String` with a renderable children object, e.g. `Children` with `render_to(&mut W)`
- removes more allocations but adds type/lifetime complexity

Start with option 1, then move to option 2 if profiling proves it worthwhile.

## Async Considerations

Avoid mixing async writer traits in phase 1.

For `html_async*_to!`, keep async component execution but write through `fmt::Write` synchronously after await points. This still removes `String` fan-out and minimizes complexity.

A later phase can add `tokio::io::AsyncWrite` support behind separate macros/adapters.

## Macro Ergonomics: Hidden vs Explicit Writer

- Fully hidden writer passing is possible inside generated code and is worth doing.
- Fully hidden top-level writer is not desirable; caller should pass writer explicitly once.

This gives a good balance:

- explicit ownership/lifetime at boundary
- near-hidden propagation for nested markup/components

## Required Changes in `freshed-rs-macros`

1. Update  `MacroMode`s .
2. Update parser for `(writer, ctx?, markup...)` argument forms.
3. Add writer-emitting codegen path in `to_html.rs`.
4. Extend `#[component]` to support writer-accepting signatures.
5. Keep old signatures as compatibility path.
6. Introduce shared `RenderError` + `RenderResult` in runtime crate.

## Suggested Migration Plan

1. Convert all components to stream-native signatures.

