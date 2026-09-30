// source: packages/tui/src/feature_plugins/sidebar/ — barrel (mod.rs)
#![allow(dead_code)]
pub mod context;
pub mod files;
pub mod footer;
pub mod lsp;
pub mod mcp;
pub mod todo;

// Re-exports (source order)
pub use context::*;
pub use files::*;
pub use footer::*;
pub use lsp::*;
pub use mcp::*;
pub use todo::*;
