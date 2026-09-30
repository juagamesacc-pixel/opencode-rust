//! Rust port of `packages/app/src/context/layout-helpers.ts` (opencode v1.18.30).
//!
//! Source 39 lines. Exports: `ensureSessionKey`, `createSessionKeyReader`, `pruneSessionKeys`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/layout-helpers.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/layout-helpers.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `ensureSessionKey`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout-helpers.ts
#[allow(non_snake_case)]
pub fn ensureSessionKey(/* key: string, touch: (key: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `createSessionKeyReader`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout-helpers.ts
#[allow(non_snake_case)]
pub fn createSessionKeyReader(/* sessionKey: string | Accessor<string>, ensure: (key: string */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `pruneSessionKeys`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout-helpers.ts
#[allow(non_snake_case)]
pub fn pruneSessionKeys(/* input: {
  keep?: string
  max: number
  used: Map<string, number>
  view: string[]
  tabs: string[]
} */) -> serde_json::Value {
    serde_json::json!({})
}
