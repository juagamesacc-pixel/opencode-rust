// source: packages/plugin/src/v2/effect/integration.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/integration.ts` (opencode v1.18.30).
//!
//! Source 63 lines. Exports: `IntegrationOAuthAuthorization`, `IntegrationOAuthMethodRegistration`, `IntegrationMethodRegistration`, `IntegrationDraft`, `IntegrationHooks`.
//!
//! PROVISIONAL: `effect` (`Effect`, `Scope`) pending effect runtime, `@opencode-ai/sdk/v2/types` pending `crates/sdk`.

use serde::{Deserialize, Serialize};

/// Mirrors `IntegrationOAuthAuthorization` verbatim fields order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationOAuthAuthorization {
    pub url: String,
    pub instructions: String,
    pub mode: String,
    pub callback: serde_json::Value,
}

/// Mirrors `IntegrationOAuthMethodRegistration` verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationOAuthMethodRegistration {
    #[serde(rename = "integrationID")]
    pub integration_id: String,
    pub method: serde_json::Value,
    pub authorize: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<serde_json::Value>,
}

/// Mirrors `IntegrationMethodRegistration` union verbatim.
pub type IntegrationMethodRegistration = serde_json::Value;

/// Mirrors `IntegrationDraft` method names verbatim.
pub const INTEGRATION_DRAFT_METHODS: &[&str] = &["list", "get", "update", "remove"];
pub const INTEGRATION_DRAFT_METHOD_LIST: &[&str] = &["list", "update", "remove"];

/// Mirrors `IntegrationDraft` brand.
pub type IntegrationDraft = serde_json::Value;

/// Mirrors `IntegrationHooks` brand with connection active/resolve verbatim.
pub type IntegrationHooks = serde_json::Value;

pub const INTEGRATION_HOOKS_CONNECTION_METHODS: &[&str] = &["active", "resolve"];

/// PROVISIONAL: `effect` pending.
pub mod effect_provisional {
    pub const PACKAGE: &str = "effect";
    pub const EFFECT: &str = "Effect.Effect";
    pub const SCOPE: &str = "Scope.Scope";
    pub const PENDING_CRATE: &str = "effect";
}
