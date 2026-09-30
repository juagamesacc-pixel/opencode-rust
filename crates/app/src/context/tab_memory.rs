//! Rust port of `packages/app/src/context/tab-memory.ts` (opencode v1.18.30).
//!
//! Source 37 lines. Exports: `createTabMemory`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/tab-memory.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/tab-memory.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `createTabMemory`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/tab-memory.ts
#[allow(non_snake_case)]
pub fn createTabMemory(/* owner: Owner | null */) -> serde_json::Value {
    serde_json::json!({})
}
