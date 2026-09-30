//! Rust port of `packages/app/src/context/server.tsx` (opencode v1.18.30).
//!
//! Source 361 lines. Exports: `RECENTLY_CLOSED_DISPLAY_LIMIT`, `normalizeServerUrl`, `serverName`, `migrateCanonicalLocalServerState`, `createServerProjects`, `resolveServerList`, `HttpBase`, `Http`, `Sidecar`, `Ssh`, `Any`, `key`, `Key`, `builtin`, `local`, `nextServerAfterRemoval`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/server.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/server.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

pub const RECENTLY_CLOSED_DISPLAY_LIMIT: i64 = 5;

/// Mirrors `normalizeServerUrl`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server.tsx
#[allow(non_snake_case)]
pub fn normalizeServerUrl(/* input: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `serverName`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server.tsx
#[allow(non_snake_case)]
pub fn serverName(/* conn?: ServerConnection.Any, ignoreDisplayName = false */) -> serde_json::Value
{
    serde_json::json!({})
}

/// Mirrors `migrateCanonicalLocalServerState`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server.tsx
#[allow(non_snake_case)]
pub fn migrateCanonicalLocalServerState(/* value: unknown, canonicalLocalServer?: ServerConnection.Key */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `createServerProjects`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server.tsx
#[allow(non_snake_case)]
pub fn createServerProjects() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `resolveServerList`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server.tsx
#[allow(non_snake_case)]
pub fn resolveServerList(/* input: {
  props?: Array<ServerConnection.Any>
  stored: StoredServer[]
} */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `HttpBase`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpBase {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Http`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Http {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Sidecar`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sidecar {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Ssh`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ssh {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Any`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Any {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `key`.
pub fn key_value() -> String {
    String::new()
}

/// Mirrors `Key`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `builtin`.
pub fn builtin_value() -> String {
    String::new()
}

/// Mirrors `local`.
pub fn local_value() -> String {
    String::new()
}

/// Mirrors `nextServerAfterRemoval`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server.tsx
#[allow(non_snake_case)]
pub fn nextServerAfterRemoval(/* 
  servers: ServerConnection.Any[],
  removed: ServerConnection.Key,
  fallback: ServerConnection.Key,
 */) -> serde_json::Value {
    serde_json::json!({})
}
