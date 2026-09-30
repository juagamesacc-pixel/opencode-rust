//! Rust port of `packages/app/src/context/terminal.tsx` (opencode v1.18.30).
//!
//! Source 547 lines. Exports: `LocalPTY`, `migrateTerminalState`, `getWorkspaceTerminalCacheKey`, `getLegacyTerminalStorageKeys`, `clearWorkspaceTerminals`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/terminal.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/terminal.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `LocalPTY`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalPTY {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `migrateTerminalState`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/terminal.tsx
#[allow(non_snake_case)]
pub fn migrateTerminalState(/* value: unknown */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `getWorkspaceTerminalCacheKey`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/terminal.tsx
#[allow(non_snake_case)]
pub fn getWorkspaceTerminalCacheKey(/* dir: string, scope: ServerScopeValue = ServerScope.local */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `getLegacyTerminalStorageKeys`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/terminal.tsx
#[allow(non_snake_case)]
pub fn getLegacyTerminalStorageKeys(/* dir: string, legacySessionID?: string */) -> serde_json::Value
{
    serde_json::json!({})
}

/// Mirrors `clearWorkspaceTerminals`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/terminal.tsx
#[allow(non_snake_case)]
pub fn clearWorkspaceTerminals(/* 
  dir: string,
  sessionIDs?: string[],
  platform?: Platform,
  scope: ServerScopeValue = ServerScope.local,
 */) -> serde_json::Value {
    serde_json::json!({})
}
