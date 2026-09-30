// source: packages/plugin/src/v2/promise/command.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/command.ts` (opencode v1.18.30).
//!
//! Source 8 lines. Re-exports `CommandDraft` from `../effect/command.js`, defines `CommandHooks` Promise variant.

pub use crate::v2::effect::command::CommandDraft;

/// Mirrors `CommandHooks = Hooks<{ transform: CommandDraft }>` Promise variant verbatim.
pub type CommandHooks = serde_json::Value;
