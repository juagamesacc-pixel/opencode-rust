#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/openai-compatible-chat.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_OPENAI_COMPATIBLE_CHAT_0: &str = "openai-compatible-chat";
pub const VERBATIM__CHAT_COMPLETIONS_1: &str = "/chat/completions";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openai_compatible_chat".to_string(),
        value: Value::Null,
    }
}
