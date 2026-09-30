//! Rust port of `packages/app/src/context/mcp.ts` (opencode v1.18.30).
//!
//! Source 20 lines. Exports: `useMcpToggle`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/mcp.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/mcp.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `useMcpToggle`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/mcp.ts
#[allow(non_snake_case)]
pub fn useMcpToggle() -> serde_json::Value {
    serde_json::json!({})
}
