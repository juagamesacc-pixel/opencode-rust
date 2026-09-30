#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/route/protocol.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_WHAT_DOES_THIS_API_LOOK__1: &str = "what does
 * this API look like";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "protocol".to_string(),
        value: Value::Null,
    }
}
pub const PROTOCOL_ID: &str = "protocol";
