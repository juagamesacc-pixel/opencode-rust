// source: packages/plugin/src/v2/options.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/options.ts` (opencode v1.18.30).
//!
//! Source 1 line: `export type PluginOptions = Readonly<Record<string, any>>`.

/// Mirrors `PluginOptions = Readonly<Record<string, any>>` verbatim.
pub type PluginOptions = serde_json::Value;
