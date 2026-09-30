//! Rust port of `packages/app/src/context/terminal-title.ts` (opencode v1.18.30).
//!
//! Source 25 lines. Exports: `defaultTitle`, `isDefaultTitle`, `titleNumber`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/terminal-title.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `defaultTitle`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/terminal-title.ts
#[allow(non_snake_case)]
pub fn defaultTitle(/* number: number */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `isDefaultTitle`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/terminal-title.ts
#[allow(non_snake_case)]
pub fn isDefaultTitle(/* title: string, number: number */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `titleNumber`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/terminal-title.ts
#[allow(non_snake_case)]
pub fn titleNumber(/* title: string, max: number */) -> serde_json::Value {
    serde_json::json!({})
}
