//! Rust port of `packages/app/src/context/permission.tsx` (opencode v1.18.30).
//!
//! Source 485 lines. Exports: none.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/permission.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/permission.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

// No exports detected — module placeholder.
pub fn placeholder() -> serde_json::Value {
    serde_json::json!({})
}
