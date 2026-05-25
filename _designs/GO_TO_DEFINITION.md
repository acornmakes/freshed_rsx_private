# Go to Definition Support

This repo adds IDE navigation support for component tags inside macro markup (for example, `<Badge {tone}>`).

## What was added

- The macro now emits hidden symbol anchors for each component tag during expansion.
- Anchors include both:
  - component function path (for example, `Badge`)
  - props type path (for example, `BadgeProps`)
- Shorthand prop parsing (`{tone}`) preserves the raw identifier token span before expression fallback to improve local-variable mapping.

## How it works

- While walking markup nodes, component tags are collected as symbol hints.
- During final expansion, the macro inserts no-op statements such as:
  - `let _ = ComponentPath;`
  - `let _: Option<PropsPath> = None;`
- These statements are span-anchored to the original tag tokens, giving rust-analyzer real Rust symbols to resolve.

## Why the namespace looks odd in fixtures

Trybuild fixtures under `tests/ui/pass` are not normal crate sources by default.
To make IDE navigation available there, a small integration-test harness includes fixture files as modules.
That is why paths can appear like `ui_pass_for_ide::pass_component_sync_props`.

## Scope and caveat

- Runtime HTML output is unchanged.
- This improves navigation where rust-analyzer uses macro expansion spans, but behavior still depends on IDE/language-server support.
