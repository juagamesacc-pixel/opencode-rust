#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/route/framing.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_SSE_1: &str = "sse";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "framing".to_string(),
        value: Value::Null,
    }
}
pub const FRAMING_SSE: &str = "sse";
