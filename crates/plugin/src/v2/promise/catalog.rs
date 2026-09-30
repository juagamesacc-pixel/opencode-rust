// source: packages/plugin/src/v2/promise/catalog.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/catalog.ts` (opencode v1.18.30).
//!
//! Source 8 lines. Re-exports `CatalogDraft`, `CatalogProviderRecord` from `../effect/catalog.js`, defines `CatalogHooks` Promise variant.

pub use crate::v2::effect::catalog::{CatalogDraft, CatalogProviderRecord};

/// Mirrors `CatalogHooks = Hooks<{ transform: CatalogDraft }>` Promise variant verbatim.
pub type CatalogHooks = serde_json::Value;
