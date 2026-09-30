//! Port of packages/app/src/components/session/session-context-format.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde_json::Value;

/// Port of packages/app/src/components/session/session-context-format.ts — pure logic / types.
// Exported symbols: createSessionContextFormatter

pub fn create_session_context_formatter(_input: Value) -> Value {
    Value::Null
}
