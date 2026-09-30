// source: packages/plugin/src/v2/promise/reference.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/reference.ts` (opencode v1.18.30).
//!
//! Source 8 lines. Re-exports `ReferenceDraft` from `../effect/reference.js`, defines `ReferenceHooks` Promise variant.

pub use crate::v2::effect::reference::ReferenceDraft;

/// Mirrors `ReferenceHooks = Hooks<{ transform: ReferenceDraft }>` Promise variant verbatim.
pub type ReferenceHooks = serde_json::Value;
