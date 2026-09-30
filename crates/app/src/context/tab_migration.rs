//! Rust port of `packages/app/src/context/tab-migration.ts` (opencode v1.18.30).
//!
//! Source 24 lines. Exports: `migrateTabs`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/tab-migration.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `migrateTabs`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/tab-migration.ts
#[allow(non_snake_case)]
pub fn migrateTabs(/* value: unknown, fallback: ServerConnection.Key */) -> serde_json::Value {
    serde_json::json!({})
}
