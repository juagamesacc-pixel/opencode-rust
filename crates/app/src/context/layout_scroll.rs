//! Rust port of `packages/app/src/context/layout-scroll.ts` (opencode v1.18.30).
//!
//! Source 127 lines. Exports: `SessionScroll`, `createScrollPersistence`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/layout-scroll.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/layout-scroll.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `SessionScroll`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionScroll {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `createScrollPersistence`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout-scroll.ts
#[allow(non_snake_case)]
pub fn createScrollPersistence(/* opts: Options */) -> serde_json::Value {
    serde_json::json!({})
}
