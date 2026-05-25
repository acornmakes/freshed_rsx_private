# Component Model Implementation Task List

Associated design: `component.md`

## 1. Goal

Implement the JSX-like, SSR-only component model described in `component.md` with:

- Component lookup at compile time
- Context/state threading (`ctx`) through nested components
- Sync and async component rendering modes
- Optional props by omission + `Default`
- Strong diagnostics at macro call sites

## 2. Non-Goals (Must Not Be Added)

- VDOM
- Hydration/runtime reconciliation
- Client state/subscriptions/signals

## 3. Delivery Strategy

Implement in vertical slices so each stage compiles and adds test coverage.

## 4. Work Items

## Phase 0: Prep and Baseline

- [x] 0.1 Add crate-level TODO/feature tracking note in macro crate comments for component pipeline.
- [x] 0.2 Add snapshot tests for current intrinsic behavior to prevent regressions.
- [x] 0.3 Add macro compile-fail test harness if not present (`trybuild` recommended).
- [x] 0.4 Define a short coding convention note for generated temp symbols (`__fr_*`).
- [x] 0.5 Ensure workspace tests run in one command and document it.

Exit criteria:

- Baseline tests pass and capture current html behavior.

## Phase 1: Public Macro Entrypoints

Files:

- `freshed-rs-macros/src/lib.rs`

Tasks:

- [x] 1.1 Keep existing `html!` behavior unchanged.
- [x] 1.2 Add `html_async!` proc macro entrypoint.
- [x] 1.3 Add `html_ctx!` proc macro entrypoint (first argument is context expression).
- [x] 1.4 Add `html_async_ctx!` proc macro entrypoint.
- [x] 1.5 Route all entrypoints through a single internal compile function with mode flags.
- [x] 1.6 Add `#[component]` declaration macro to support snake_case component function definitions.

Exit criteria:

- New macro names compile and dispatch into shared internal pipeline.

## Phase 2: Input Parsing for Context + Markup

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 2.1 Introduce input parser type for all macro forms.
- [x] 2.2 Parse context-first forms (`html_ctx!`, `html_async_ctx!`) as `(ctx_expr, rstml_tokens)`.
- [x] 2.3 Parse context-free forms as `rstml_tokens` only.
- [x] 2.4 Emit clear diagnostics for missing context argument in `_in` macros.
- [x] 2.5 Reject trailing garbage tokens after parsed markup with precise spans.

Exit criteria:

- Entrypoint parsing is deterministic and mode-specific diagnostics are emitted.

## Phase 3: Internal IR and Mode Flags

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 3.1 Add compile mode enum:
  - SyncNoCtx
  - AsyncNoCtx
  - SyncWithCtx
  - AsyncWithCtx
- [x] 3.2 Extend current output IR to store expression fragments without losing existing format strategy.
- [x] 3.3 Thread compile mode through all visitor/codegen paths.
- [x] 3.4 Ensure no behavior change for intrinsic-only trees in SyncNoCtx mode.

Exit criteria:

- Single compiler path supports all four modes.

## Phase 4: Component Tag Classification and Path Handling

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 4.1 Implement `is_component_tag(NodeName) -> bool` using design rules.
- [x] 4.2 Implement component path extraction preserving source spans.
- [x] 4.3 Implement props type inference (`Foo` -> `FooProps`, `ui::Foo` -> `ui::FooProps`).
- [x] 4.4 Keep intrinsic branch untouched for lowercase/html/custom-element tags.

Exit criteria:

- Component tags are reliably split from intrinsic tags.

## Phase 5: Component Attribute Parsing

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 5.1 Parse `key="literal"` component attributes.
- [x] 5.2 Parse `key={expr}` component attributes.
- [x] 5.3 Parse shorthand `{ident}` into `ident={ident}`.
- [x] 5.4 Reject shorthand non-ident blocks (`{a + b}`) with targeted diagnostic.
- [x] 5.5 Detect duplicate prop keys and emit hard error.
- [x] 5.6 Reserve and track `children` key explicitly for conflict checks.

Exit criteria:

- Parsed props map is stable and diagnostics are source-accurate.

## Phase 6: Children Compilation and Injection

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 6.1 Compile child node list into string expression via existing static+values path.
- [x] 6.2 Inject `children` field when child nodes are present.
- [x] 6.3 Inject `children: String::new()` when absent and no explicit `children` prop.
- [x] 6.4 Emit hard error when both explicit `children` prop and child nodes are present.

Exit criteria:

- Child rendering works for intrinsic + nested component trees.

## Phase 7: Context Threading (State Propagation)

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 7.1 In `*_in` modes, emit component calls as `ComponentPath(ctx_expr, PropsPath { ... })`.
- [x] 7.2 In non-context modes, emit component calls as `ComponentPath(PropsPath { ... })`.
- [x] 7.3 Preserve hygienic capture of `ctx_expr` (single evaluation).
- [x] 7.4 Validate nested components automatically receive the same `ctx_expr`.

Exit criteria:

- Request-scoped state is consistently threaded through all component calls.

## Phase 8: Async Rendering Pipeline

Files:

- `freshed-rs-macros/src/lib.rs`
- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 8.1 Add async mode codegen wrapper (`async move { ... }`).
- [x] 8.2 Support `async` marker on component tags in async modes only.
- [x] 8.3 Expand marked calls with `.await`.
- [x] 8.4 Emit hard error when `async` marker appears in sync modes.
- [x] 8.5 Keep output order deterministic in mixed sync/async component trees.

Exit criteria:

- Async component calls are non-blocking at runtime and valid only in async macro modes.

## Phase 9: Optional Props and Struct Construction

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 9.1 Generate struct literal with only provided fields + generated children field.
- [x] 9.2 Always append `..::core::default::Default::default()`.
- [x] 9.3 Confirm omitted optional fields (`Option<T>`) resolve to default (`None`).
- [ ] 9.4 Confirm required field omissions are delegated to Rust compile errors. (Skipped: conflicts with the Default-based component construction path.)

Exit criteria:

- Optional vs required prop behavior matches design.

## Phase 10: Escaping and SSR Safety

Files:

- `freshed-rs-macros/src/to_html.rs`
- Optional future: separate runtime utility crate if needed

Tasks:

- [x] 10.1 Define escaping rules for text node interpolation.
- [x] 10.2 Define escaping rules for attribute interpolation.
- [x] 10.3 Define raw trusted-html escape hatch type contract.
- [x] 10.4 Ensure no double-escaping for trusted-html wrapper values.
- [x] 10.5 Add tests for XSS-sensitive interpolation cases.

Exit criteria:

- Interpolated output is safe by default for SSR.

## Phase 11: IDE and Diagnostics Quality

Files:

- `freshed-rs-macros/src/to_html.rs`

Tasks:

- [x] 11.1 Remove the separate IDE helper macro and keep shared HTML behavior in `html!`.
- [ ] 11.2 Ensure component-related diagnostics include tag/prop names.
- [ ] 11.3 Ensure all macro-owned diagnostics use source spans from original syntax.
- [ ] 11.4 Add diagnostics for malformed context macro invocations.

Exit criteria:

- Error messages are actionable and attributed to source call sites.

## Phase 12: Tests

Files:

- `examples/src/main.rs`
- New tests in macro crate (`tests/` suggested)

Tasks:

- [ ] 12.1 Positive: simple sync component.
- [ ] 12.2 Positive: nested components.
- [ ] 12.3 Positive: path-qualified component.
- [ ] 12.4 Positive: shorthand props.
- [ ] 12.5 Positive: optional prop omitted.
- [ ] 12.6 Positive: context propagation through nested tree.
- [ ] 12.7 Positive: async component in `html_async!`.
- [ ] 12.8 Positive: async component + context in `html_async_ctx!`.
- [ ] 12.9 Negative: duplicate prop key.
- [ ] 12.10 Negative: mixed `children` prop + body children.
- [ ] 12.11 Negative: invalid shorthand block.
- [ ] 12.12 Negative: async marker in sync macro.
- [ ] 12.13 Negative: missing context in `_in` macro.
- [ ] 12.14 Negative: wrong context type (Rust error).
- [ ] 12.15 Negative: unknown prop field (Rust error).
- [ ] 12.16 Negative: missing required prop (Rust error).

Exit criteria:

- Full positive/negative matrix is automated and green.

## Phase 13: Examples and Documentation

Files:

- `examples/src/main.rs`
- `_designs/component.md`
- `README.md` (if present)

Tasks:

- [ ] 13.1 Add minimal context-aware SSR example (db/session/user fields mocked).
- [ ] 13.2 Add async SSR example with simulated IO.
- [ ] 13.3 Document migration guidance from `html!` to `html_ctx!`.
- [ ] 13.4 Document optional prop convention (`Option<T>` + `Default`).

Exit criteria:

- Developers can adopt sync, async, and context-aware flows from examples alone.

## 5. Suggested Build Order (Command Checklist)

- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo test -p freshed-rs-examples`
- [ ] `cargo test -p freshed-rs-macros`

## 6. Implementation Notes

- Keep edits additive and preserve current intrinsic behavior first.
- Avoid broad refactors before Phase 3 scaffolding is in place.
- Treat diagnostics quality as a feature, not cleanup.
- Prefer compile-time errors over runtime panics.
- Idiomatic Rust Always
- Avoid uses of panic! or unwrap whenever possible. Only use if necessary. 


## 7. Definition of Done

All items below must be true:

- [ ] All phase exit criteria are satisfied.
- [ ] All tests in Phase 12 are implemented and passing.
- [ ] No regressions for existing intrinsic `html!` behavior.
- [ ] Context/state propagation is demonstrated in examples.
- [ ] Async rendering path is demonstrated and tested.
- [ ] No VDOM/hydration/state runtime has been introduced.
