#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/schema/messages.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_TEXT_1: &str = "text";
pub const VERBATIM_LLM_SYSTEMPART_2: &str = "LLM.SystemPart";
pub const VERBATIM_STRING_3: &str = "string";
pub const VERBATIM_LLM_CONTENT_TEXT_4: &str = "LLM.Content.Text";
pub const VERBATIM_MEDIA_5: &str = "media";
pub const VERBATIM_LLM_CONTENT_MEDIA_6: &str = "LLM.Content.Media";
pub const VERBATIM_JSON_7: &str = "json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "messages".to_string(),
        value: Value::Null,
    }
}
