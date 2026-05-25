#![allow(dead_code)]

// This file exists to make ui/pass fixtures part of a real Cargo target so
// rust-analyzer can provide navigation in those files.
#[path = "ui/pass/pass_component_sync_props.rs"]
mod pass_component_sync_props;
