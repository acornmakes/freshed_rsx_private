#![allow(dead_code)]

// This file exists to make ui_stream/pass fixtures part of a real Cargo target so
// rust-analyzer can provide navigation in those files.
#[path = "ui_stream/pass/component_ctx_async.rs"]
mod component_ctx_async;
