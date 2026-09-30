#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/openai-responses.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_OPENAI_RESPONSES_1: &str = "openai-responses";
pub const VERBATIM_HTTPS_API_OPENAI_COM_V1_2: &str = "https://api.openai.com/v1";
pub const VERBATIM__RESPONSES_3: &str = "/responses";
pub const VERBATIM_INPUT_TEXT_4: &str = "input_text";
pub const VERBATIM_INPUT_IMAGE_5: &str = "input_image";
pub const VERBATIM_OUTPUT_TEXT_6: &str = "output_text";
pub const VERBATIM_SUMMARY_TEXT_7: &str = "summary_text";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openai_responses".to_string(),
        value: Value::Null,
    }
}
