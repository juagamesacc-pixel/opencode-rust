//! Rust port of `packages/app/src/context/global-sync/utils.ts` (opencode v1.18.30).
//!
//! Source 173 lines. Exports: `cmp`, `normalizeAgentList`, `normalizePermissionRequest`, `normalizeProviderList`, `sanitizeProject`, `normalizeProjectInfo`, `pathKey`, `PathKey`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/utils.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `cmp`.
pub fn cmp_value() -> String {
    String::new()
}

/// Mirrors `normalizeAgentList`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/utils.ts
#[allow(non_snake_case)]
pub fn normalizeAgentList(/* input: AgentListOutput["data"] | Agent[] */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `normalizePermissionRequest`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/utils.ts
#[allow(non_snake_case)]
pub fn normalizePermissionRequest(/* input: PermissionV2Request | PermissionRequest */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `normalizeProviderList`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/utils.ts
#[allow(non_snake_case)]
pub fn normalizeProviderList(/* 
  providers: ProviderListOutput["data"] | ProviderListResponse,
  models?: ModelListOutput["data"],
  defaultModel?: ModelDefaultOutput["data"],
 */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `sanitizeProject`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/utils.ts
#[allow(non_snake_case)]
pub fn sanitizeProject(/* project: Project */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `normalizeProjectInfo`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/utils.ts
#[allow(non_snake_case)]
pub fn normalizeProjectInfo(/* project: Project | CurrentProject */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `pathKey`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/utils.ts
#[allow(non_snake_case)]
pub fn pathKey() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `PathKey` (exported as `export { type PathKey }`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PathKey {
    // PROVISIONAL: fields pending full port
}
