// source: packages/plugin/src/v2/effect/skill.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/skill.ts` (opencode v1.18.30).
//!
//! Source 11 lines. Exports: `SkillDraft`, `SkillHooks`.

/// Mirrors `SkillDraft` methods verbatim.
pub const SKILL_DRAFT_METHODS: &[&str] = &["source", "list"];

/// Mirrors `SkillDraft` brand.
pub type SkillDraft = serde_json::Value;

/// Mirrors `SkillHooks = Hooks<{ transform: SkillDraft }>` verbatim.
pub type SkillHooks = serde_json::Value;

/// PROVISIONAL: `@opencode-ai/sdk/v2/types` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2/types";
    pub const SKILL_V2_SOURCE: &str = "SkillV2Source";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
