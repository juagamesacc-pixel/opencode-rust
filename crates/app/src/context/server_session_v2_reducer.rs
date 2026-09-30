//! Rust port of `packages/app/src/context/server-session-v2-reducer.ts` (opencode v1.18.30).
//!
//! Source 510 lines. Exports: `V2SessionReduction`, `createV2SessionReducer`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/server-session-v2-reducer.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `V2SessionReduction`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2SessionReduction {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `createV2SessionReducer`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-session-v2-reducer.ts
#[allow(non_snake_case)]
pub fn createV2SessionReducer() -> serde_json::Value {
    serde_json::json!({})
}
