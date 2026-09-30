//! Port of packages/app/src/components/session/session-context-metrics.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde_json::Value;

/// Port of packages/app/src/components/session/session-context-metrics.ts — pure logic / types.
// Exported symbols: getSessionContext
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/session/session-context-metrics.ts

pub fn get_session_context(_input: Value) -> Value {
    Value::Null
}
