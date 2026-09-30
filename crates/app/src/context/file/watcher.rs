//! Rust port of `packages/app/src/context/file/watcher.ts` (opencode v1.18.30).
//!
//! Source 54 lines. Exports: `invalidateFromWatcher`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/file/watcher.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `invalidateFromWatcher`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/watcher.ts
#[allow(non_snake_case)]
pub fn invalidateFromWatcher(/* event: WatcherEvent, ops: WatcherOps */) -> serde_json::Value {
    serde_json::json!({})
}
