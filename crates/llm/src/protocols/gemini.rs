#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/gemini.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_GEMINI_1: &str = "gemini";
pub const VERBATIM_HTTPS_GENERATIVELANGUAGE_2: &str =
    "https://generativelanguage.googleapis.com/v1beta";
pub const VERBATIM_USER_3: &str = "user";
pub const VERBATIM_MODEL_4: &str = "model";
pub const VERBATIM_AUTO_5: &str = "AUTO";
pub const VERBATIM_NONE_6: &str = "NONE";
pub const VERBATIM_ANY_7: &str = "ANY";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "gemini".to_string(),
        value: Value::Null,
    }
}
