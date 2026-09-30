//! Rust port of `packages/core/src/config` barrel.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! Barrel mirrors `packages/core/src/config/*.ts` lexical order (no index.ts in source).

pub mod agent;
pub mod attachments;
pub mod command;
pub mod compaction;
pub mod experimental;
pub mod formatter;
pub mod lsp;
pub mod markdown;
pub mod mcp;
pub mod plugin;
pub mod provider;
pub mod reference;
pub mod tool_output;
pub mod watcher;
