// source: packages/plugin/src/v2/effect/catalog.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/catalog.ts` (opencode v1.18.30).
//!
//! Source 29 lines. Exports: `CatalogProviderRecord`, `CatalogDraft`, `CatalogHooks`.
//!
//! PROVISIONAL: `@opencode-ai/sdk/v2/types` (`ModelV2Info`, `ProviderV2Info`) pending `crates/sdk`.

/// Mirrors `CatalogProviderRecord` verbatim.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CatalogProviderRecord {
    pub provider: serde_json::Value,
    pub models: serde_json::Value,
}

/// Mirrors `CatalogDraft.provider` methods verbatim.
pub const CATALOG_PROVIDER_METHODS: &[&str] = &["list", "get", "update", "remove"];

/// Mirrors `CatalogDraft.model` methods verbatim.
pub const CATALOG_MODEL_METHODS: &[&str] = &["get", "update", "remove"];

/// Mirrors `CatalogDraft.model.default` methods verbatim.
pub const CATALOG_MODEL_DEFAULT_METHODS: &[&str] = &["get", "set"];

/// Mirrors `CatalogDraft` brand.
pub type CatalogDraft = serde_json::Value;

/// Mirrors `CatalogHooks = Hooks<{ transform: CatalogDraft }>` verbatim.
pub type CatalogHooks = serde_json::Value;

/// Mirrors `ModelV2Info` / `ProviderV2Info` brands.
pub type ModelV2Info = serde_json::Value;
pub type ProviderV2Info = serde_json::Value;

/// PROVISIONAL: `@opencode-ai/sdk/v2/types` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2/types";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
