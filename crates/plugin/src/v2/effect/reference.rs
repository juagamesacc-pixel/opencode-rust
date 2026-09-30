// source: packages/plugin/src/v2/effect/reference.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/reference.ts` (opencode v1.18.30).
//!
//! Source 12 lines. Exports: `ReferenceDraft`, `ReferenceHooks`.

/// Mirrors `ReferenceDraft` methods verbatim.
pub const REFERENCE_DRAFT_METHODS: &[&str] = &["add", "remove", "list"];

/// Mirrors `ReferenceDraft` brand.
pub type ReferenceDraft = serde_json::Value;

/// Mirrors `ReferenceHooks = Hooks<{ transform: ReferenceDraft }>` verbatim.
pub type ReferenceHooks = serde_json::Value;

/// PROVISIONAL: `@opencode-ai/sdk/v2/types` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2/types";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
