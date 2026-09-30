//! Rust port of `packages/app/src/context/model-variant.ts` (opencode v1.18.30).
//!
//! Source 53 lines. Exports: `getConfiguredAgentVariant`, `resolveModelVariant`, `cycleModelVariant`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/model-variant.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `getConfiguredAgentVariant`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/model-variant.ts
#[allow(non_snake_case)]
pub fn getConfiguredAgentVariant(/* input: { agent: Agent | undefined; model: Model | undefined } */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `resolveModelVariant`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/model-variant.ts
#[allow(non_snake_case)]
pub fn resolveModelVariant(/* input: VariantInput */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `cycleModelVariant`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/model-variant.ts
#[allow(non_snake_case)]
pub fn cycleModelVariant(/* input: VariantInput */) -> serde_json::Value {
    serde_json::json!({})
}
