// source: packages/plugin/src/v2/effect/command.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/command.ts` (opencode v1.18.30).
//!
//! Source 13 lines. Exports: `CommandDraft`, `CommandHooks`.
//!
//! PROVISIONAL: `@opencode-ai/sdk/v2/types` (`CommandV2Info`) pending `crates/sdk`.

/// Mirrors `CommandDraft` methods verbatim.
pub const COMMAND_DRAFT_METHODS: &[&str] = &["list", "get", "update", "remove"];

/// Mirrors `CommandDraft` brand.
pub type CommandDraft = serde_json::Value;

/// Mirrors `CommandHooks = Hooks<{ transform: CommandDraft }>` verbatim.
pub type CommandHooks = serde_json::Value;

/// Mirrors `CommandV2Info` brand.
pub type CommandV2Info = serde_json::Value;

/// PROVISIONAL: `@opencode-ai/sdk/v2/types` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2/types";
    pub const COMMAND_V2_INFO: &str = "CommandV2Info";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
