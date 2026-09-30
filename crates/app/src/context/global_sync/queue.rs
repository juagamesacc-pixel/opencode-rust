//! Rust port of `packages/app/src/context/global-sync/queue.ts` (opencode v1.18.30).
//!
//! Source 88 lines. Exports: `createRefreshQueue`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/queue.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `createRefreshQueue`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/queue.ts
#[allow(non_snake_case)]
pub fn createRefreshQueue(/* input: QueueInput */) -> serde_json::Value {
    serde_json::json!({})
}
