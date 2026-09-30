//! Rust port of `packages/app/src/context/global-sync/session-trim.ts` (opencode v1.18.30).
//!
//! Source 58 lines. Exports: `sessionUpdatedAt`, `compareSessionRecent`, `takeRecentSessions`, `trimSessions`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/session-trim.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `sessionUpdatedAt`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/session-trim.ts
#[allow(non_snake_case)]
pub fn sessionUpdatedAt(/* session: Session */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `compareSessionRecent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/session-trim.ts
#[allow(non_snake_case)]
pub fn compareSessionRecent(/* a: Session, b: Session */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `takeRecentSessions`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/session-trim.ts
#[allow(non_snake_case)]
pub fn takeRecentSessions(/* sessions: Session[], limit: number, cutoff: number */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `trimSessions`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/session-trim.ts
#[allow(non_snake_case)]
pub fn trimSessions(/* 
  input: Session[],
  options: { limit: number; permission: Record<string, PermissionRequest[]>; now?: number },
 */) -> serde_json::Value {
    serde_json::json!({})
}
