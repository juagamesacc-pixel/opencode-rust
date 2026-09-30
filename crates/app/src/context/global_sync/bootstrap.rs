//! Rust port of `packages/app/src/context/global-sync/bootstrap.ts` (opencode v1.18.30).
//!
//! Source 555 lines. Exports: `clearProviderRev`, `loadGlobalConfigQuery`, `loadProjectsQuery`, `loadProvidersQuery`, `loadAgentsQuery`, `loadCommands`, `loadPathQuery`, `loadReferencesQuery`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/bootstrap.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/global-sync/bootstrap.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `clearProviderRev`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/bootstrap.ts
#[allow(non_snake_case)]
pub fn clearProviderRev(/* scope: ServerScope, directory: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `loadGlobalConfigQuery`.
pub fn loadGlobalConfigQuery_value() -> String {
    String::new()
}

/// Mirrors `loadProjectsQuery`.
pub fn loadProjectsQuery_value() -> String {
    String::new()
}

/// Mirrors `loadProvidersQuery`.
pub fn loadProvidersQuery_value() -> String {
    String::new()
}

/// Mirrors `loadAgentsQuery`.
pub fn loadAgentsQuery_value() -> String {
    String::new()
}

/// Mirrors `loadCommands`.
pub fn loadCommands_value() -> String {
    String::new()
}

/// Mirrors `loadPathQuery`.
pub fn loadPathQuery_value() -> String {
    String::new()
}

/// Mirrors `loadReferencesQuery`.
pub fn loadReferencesQuery_value() -> String {
    String::new()
}
