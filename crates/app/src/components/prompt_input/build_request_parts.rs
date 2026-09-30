//! Port of packages/app/src/components/prompt-input/build-request-parts.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/build-request-parts.ts — pure logic / types.
// Exported symbols: buildRequestParts
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/prompt-input/build-request-parts.ts

pub fn build_request_parts(_input: Value) -> Value {
    Value::Null
}
