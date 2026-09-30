// source: packages/tui/src/plugin/ — barrel (mod.rs)
#![allow(dead_code)]
pub mod adapters;
pub mod api;
pub mod command_shim;
pub mod runtime;
pub mod slots;

// Re-exports (source order)
pub use adapters::*;
pub use api::*;
pub use command_shim::*;
pub use runtime::*;
pub use slots::*;
