// source: packages/plugin/src/v2/effect/agent.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/agent.ts` (opencode v1.18.30).
//!
//! Source 14 lines. Exports: `AgentDraft`, `AgentHooks`.
//!
//! PROVISIONAL: `@opencode-ai/sdk/v2/types` (`AgentV2Info`) pending `crates/sdk` — stub via `serde_json::Value`.

/// Mirrors `AgentDraft` interface verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct AgentDraft;

impl AgentDraft {
    pub const LIST: &'static str = "list";
    pub const GET: &'static str = "get";
    pub const DEFAULT: &'static str = "default";
    pub const UPDATE: &'static str = "update";
    pub const REMOVE: &'static str = "remove";
}

/// Mirrors `AgentDraft` method signatures as constants for parity.
pub const AGENT_DRAFT_METHODS: &[&str] = &["list", "get", "default", "update", "remove"];

/// Mirrors `AgentHooks = Hooks<{ transform: AgentDraft }>` verbatim.
pub type AgentHooks = serde_json::Value;

/// Mirrors `AgentV2Info` SDK type brand.
pub type AgentV2Info = serde_json::Value;

/// PROVISIONAL: `@opencode-ai/sdk/v2/types` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2/types";
    pub const AGENT_V2_INFO: &str = "AgentV2Info";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
