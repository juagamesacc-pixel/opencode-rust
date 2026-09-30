// source: packages/plugin/src/v2/promise/agent.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/agent.ts` (opencode v1.18.30).
//!
//! Source 8 lines. Re-exports `AgentDraft` from `../effect/agent.js`, defines `AgentHooks = Hooks<{ transform: AgentDraft }>` (Promise variant).

pub use crate::v2::effect::agent::AgentDraft;

/// Mirrors `AgentHooks = Hooks<{ transform: AgentDraft }>` Promise variant verbatim.
pub type AgentHooks = serde_json::Value;
