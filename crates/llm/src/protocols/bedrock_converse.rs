#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/bedrock-converse.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_BEDROCK_CONVERSE_1: &str = "bedrock-converse";
pub const VERBATIM_SUCCESS_2: &str = "success";
pub const VERBATIM_ERROR_3: &str = "error";
pub const VERBATIM_USER_4: &str = "user";
pub const VERBATIM_ASSISTANT_5: &str = "assistant";
pub const VERBATIM_ROLE_6: &str = "role";
pub const VERBATIM_TOOLCHOICE_7: &str = "toolChoice";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "bedrock_converse".to_string(),
        value: Value::Null,
    }
}
