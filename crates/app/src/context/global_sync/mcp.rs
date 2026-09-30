//! Rust port of `packages/app/src/context/global-sync/mcp.ts` (opencode v1.18.30).
//!
//! Source 20 lines. Exports: none.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/mcp.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

// No exports detected — module placeholder.
pub fn placeholder() -> serde_json::Value {
    serde_json::json!({})
}
