# Ideas: Make Freshed Most Useful for Axum/Actix SSR

## Product Direction

Freshed should position itself as:

- Rust-first SSR that is faster and more predictable than Node template/SSR pipelines.
- HTML-first output with native web-component support and no hydration requirement.
- Compile-time checked component ergonomics that feel familiar to JSX users.

Non-goal to keep explicit: do not become a client-side framework or VDOM runtime.

## Highest-Value Features for Axum and Actix Users

## 1) First-Class Framework Adapters (Priority: P0)

Add dedicated integration crates:

- `freshed-rs-axum`
- `freshed-rs-actix`

Core features:

- `IntoResponse` wrappers for `HtmlFragment`, stream writers, and render errors.
- Optional compressed responses and configurable content-type headers.
- Helper functions for common patterns (`render_page`, `render_partial`, `render_stream`).
- Standardized error mapping (`RenderError` -> 500 response with override hooks).

Axum-specific:

- `FromRequestParts` extraction helpers for render context.
- `tower::Layer` integration for request metadata injection (request id, auth user, locale, feature flags).

Actix-specific:

- Extractor and middleware helpers for `HttpRequest`-scoped render context.
- `Responder` helpers for full-page, partial, and chunked HTML responses.

## 2) True Streaming HTML Responses (Priority: P0)

Writer-based rendering exists; expose end-to-end HTTP streaming adapters:

- Adapt `fmt::Write` rendering to chunked HTTP body streams.
- Allow early flushing of shell HTML (`<head>`, header, skeleton), then stream body chunks.
- Support async components inside streams without buffering full page output.

Why this matters:

- Better TTFB for dynamic pages.
- Lower peak memory for large lists/tables.
- Closer to the developer experience people like in modern Node SSR streaming.

## 3) Request Context and Dependency Injection Story (Priority: P0)

Standardize the `ctx` pattern for web apps:

- A recommended `RenderContext` shape and trait-based extension points.
- Context composition helpers for db, session, user, locale, csrf nonce, csp nonce.
- Per-request context propagation from middleware to components with zero global state.

This should be documented as a first-class pattern, not just an advanced option.

## 4) Fragment and Partial Rendering for HTMX/Turbo (Priority: P1)

Web teams often need partial HTML responses, not only full pages:

- Explicit partial-render API (`render_fragment`) with shared layout context.
- Helpers for out-of-band swaps and progressive enhancement workflows.
- Stable conventions for returning either full page or partial depending on headers.

This is a major practical differentiator for Rust SSR adoption.

## Macro and Runtime Optimizations

## 5) Reduce Allocations in Children and Dynamic Blocks (Priority: P0/P1)

Current children handling can require temporary `String` buffers for component children.
Optimization ideas:

- Introduce a `Children` render object with `render_to(out)` to avoid intermediate strings.
- Add a fast-path for static children chunks.
- Keep compatibility mode for string children during migration.

## 6) Compile-Time Static Segment Hoisting (Priority: P1)

For repeated templates/components:

- Hoist static literal segments to `const` where feasible.
- Precompute static open/close tag fragments.
- Keep dynamic values as writes only.

Expected gain: lower CPU for hot render paths and fewer runtime concatenations.

## 7) Attribute Rendering Fast Paths (Priority: P1)

Attributes are a hot area in SSR:

- Separate boolean attr path (`disabled`, `checked`) from normal attr formatting.
- Micro-optimized escaping for common safe ASCII spans.
- Specialized codegen for common scalar types (`&str`, `String`, integers, bool).

## 8) Macro Expansion Performance and Diagnostics (Priority: P1)

For large codebases, compile speed and error quality matter:

- Incremental-friendly macro internals with fewer repeated parses.
- Better diagnostics for missing required props, unknown props, duplicate props.
- Suggestion diagnostics for casing/path errors (`<user_card>` vs `<UserCard>`).
- Optional lint-like warnings for suspicious patterns (unused children).

## Native Web-Component Excellence

## 9) Web-Component Attribute and Slot Ergonomics (Priority: P0/P1)

SSR output should make web components effortless:

- Clear support for kebab-case attributes and `data-*`/`aria-*` pass-through.
- Typed helpers for common attribute kinds (string/boolean/token-list/json).
- First-class slot ergonomics (`slot="name"` helpers and examples).
- Raw JSON/script payload helpers with safe escaping for component bootstrap data.

## 10) Declarative Shadow-DOM and Template Patterns (Priority: P2)

Optional advanced support:

- Helpers/patterns for declarative shadow DOM output where target browsers allow it.
- Guidance for component islands that are native custom elements, not framework hydration.

## Compare to Preact/Node SSR: What to Match and What to Beat

Developers coming from Preact-like SSR expect:

- Simple component composition.
- Streaming response support.
- Partial rendering patterns.
- Good docs and examples.

Freshed should beat Node stacks on:

- Compile-time correctness (props, components, escaping behavior).
- Throughput and memory under load.
- Operational predictability (single binary, no JS runtime in production path).

Freshed should match ergonomics by adding:

- One-command integration examples for Axum/Actix.
- Familiar developer patterns (layout components, partials, slots, async data components).
- Clear migration guide: JSX mental model -> Freshed mental model.

## Security and Correctness Features

## 11) Security Defaults and APIs (Priority: P0)

- Strict escaping by default with explicit raw-html opt-in.
- CSP nonce threading in render context with helper APIs.
- Safe JSON embedding helper for script tags.
- Optional sanitized-html integration points for trusted markdown pipelines.

## 12) Cache Semantics (Priority: P1)

- ETag helper generation for deterministic fragments/pages.
- Cache-key utilities tied to context dimensions (locale, auth state, feature flags).
- Optional fragment cache adapter traits (in-memory/redis).

## Developer Experience and Adoption

## 13) Testing Toolkit (Priority: P0/P1)

- Snapshot-testing helpers for SSR output.
- Assert helpers for HTML structure (tag/attr presence, normalized whitespace).
- Integration test utilities for Axum/Actix handlers returning HTML.

## 14) Benchmark Suite and Published Numbers (Priority: P0)

Create reproducible benchmarks:

- Render throughput and p95 latency versus selected Node SSR baselines.
- Memory allocation profiles for common page shapes.
- Streaming TTFB comparisons.

Publish results and methodology to build trust.

## 15) Documentation and Starter Templates (Priority: P0)

Provide polished docs with:

- Axum quickstart template.
- Actix quickstart template.
- Web-components-first example app (forms, tables, partial updates).
- Performance guide and anti-patterns.

## Suggested Roadmap

## Next 30 Days

- Ship `freshed-rs-axum` and `freshed-rs-actix` minimal adapters.
- Add end-to-end streaming response APIs.
- Publish one production-style example per framework.

## Next 60 Days

- Improve diagnostics for props and component tag mistakes.
- Add testing toolkit and snapshot utilities.
- Add security helper APIs (CSP nonce, safe JSON embedding).

## Next 90 Days

- Introduce children render-object optimization path.
- Add cache and partial-render helper layer.
- Publish benchmark report against Node SSR baselines.

## Success Metrics

- Time-to-first-render integration in a new Axum/Actix app under 15 minutes.
- Streaming TTFB improvement over non-streamed baseline.
- Lower allocations/page after children optimization.
- Reduced macro-related support issues due to better diagnostics.
- Community adoption of official starter templates.
