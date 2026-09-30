// source: packages/plugin/src/v2/promise/skill.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/skill.ts` (opencode v1.18.30).
//!
//! Source 8 lines. Re-exports `SkillDraft` from `../effect/skill.js`, defines `SkillHooks` Promise variant.

pub use crate::v2::effect::skill::SkillDraft;

/// Mirrors `SkillHooks = Hooks<{ transform: SkillDraft }>` Promise variant verbatim.
pub type SkillHooks = serde_json::Value;
