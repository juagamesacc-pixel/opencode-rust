// source: packages/plugin/src/v2/promise/integration.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/integration.ts` (opencode v1.18.30).
//!
//! Source 14 lines. Re-exports `IntegrationDraft`, `IntegrationMethodRegistration` from `../effect/integration.js`, defines `IntegrationHooks` Promise variant with `connection.active/resolve` returning Promise.

pub use crate::v2::effect::integration::{IntegrationDraft, IntegrationMethodRegistration};

/// Mirrors `IntegrationHooks` Promise variant brand with `connection.active/resolve` verbatim.
pub type IntegrationHooks = serde_json::Value;

pub const INTEGRATION_HOOKS_CONNECTION_METHODS: &[&str] = &["active", "resolve"];
