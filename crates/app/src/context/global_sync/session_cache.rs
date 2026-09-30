//! Rust port of `packages/app/src/context/global-sync/session-cache.ts` (opencode v1.18.30).
//!
//! Source 63 lines. Exports: `SESSION_CACHE_LIMIT`, `dropSessionCaches`, `pickSessionCacheEvictions`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/session-cache.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

pub const SESSION_CACHE_LIMIT: i64 = 40;

/// Mirrors `dropSessionCaches`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/session-cache.ts
#[allow(non_snake_case)]
pub fn dropSessionCaches(/* store: SessionCache, sessionIDs: Iterable<string> */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `pickSessionCacheEvictions`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/session-cache.ts
#[allow(non_snake_case)]
pub fn pickSessionCacheEvictions(/* input: {
  seen: Set<string>
  keep: string
  limit: number
  preserve?: Iterable<string>
} */) -> serde_json::Value {
    serde_json::json!({})
}
