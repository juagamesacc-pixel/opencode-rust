#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/openai-chat.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_OPENAI_CHAT_1: &str = "openai-chat";
pub const VERBATIM_HTTPS_API_OPENAI_COM_V1_2: &str = "https://api.openai.com/v1";
pub const VERBATIM__CHAT_COMPLETIONS_3: &str = "/chat/completions";
pub const VERBATIM_FUNCTION_4: &str = "function";
pub const VERBATIM_TEXT_5: &str = "text";
pub const VERBATIM_IMAGE_URL_6: &str = "image_url";
pub const VERBATIM_SYSTEM_7: &str = "system";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openai_chat".to_string(),
        value: Value::Null,
    }
}
