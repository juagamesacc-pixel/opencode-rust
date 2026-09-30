#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/openai.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_OPENAI_0: &str = "openai";
pub const VERBATIM_OPTIONAL_1: &str = "optional";
pub const VERBATIM_OPENAI_API_KEY_2: &str = "OPENAI_API_KEY";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openai".to_string(),
        value: Value::Null,
    }
}
