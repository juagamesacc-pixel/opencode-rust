//! Rust port of `packages/app/src/context/global-sync/child-store.ts` (opencode v1.18.30).
//!
//! Source 397 lines. Exports: `createChildStoreManager`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/child-store.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/global-sync/child-store.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `createChildStoreManager`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/child-store.ts
#[allow(non_snake_case)]
pub fn createChildStoreManager(/* input: {
  owner: Owner
  scope: ServerScope
  persist: typeof persisted
  isBooting: (directory: string */) -> serde_json::Value {
    serde_json::json!({})
}
