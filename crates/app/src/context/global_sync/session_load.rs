//! Rust port of `packages/app/src/context/global-sync/session-load.ts` (opencode v1.18.30).
//!
//! Source 34 lines. Exports: `estimateRootSessionTotal`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/session-load.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `estimateRootSessionTotal`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/session-load.ts
#[allow(non_snake_case)]
pub fn estimateRootSessionTotal(/* input: { count: number; limit: number; limited: boolean } */
) -> serde_json::Value {
    serde_json::json!({})
}
