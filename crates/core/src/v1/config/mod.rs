//! Rust port of `packages/core/src/v1/config` barrel.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! Barrel mirrors `packages/core/src/v1/config/*.ts` lexical order (no index.ts in source).

pub mod agent;
pub mod attachment;
pub mod command;
pub mod config;
pub mod console_state;
pub mod error;
pub mod formatter;
pub mod layout;
pub mod lsp;
pub mod mcp;
pub mod migrate;
pub mod permission;
pub mod plugin;
pub mod provider;
pub mod provider_options;
pub mod server;
pub mod skills;
