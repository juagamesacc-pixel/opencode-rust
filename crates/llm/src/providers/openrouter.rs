#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/openrouter.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_OPENROUTER_1: &str = "openrouter";
pub const VERBATIM_PROVIDEROPTIONS_2: &str = "providerOptions";
pub const VERBATIM_OPTIONAL_3: &str = "optional";
pub const VERBATIM_OPENROUTER_CHAT_4: &str = "openrouter-chat";
pub const VERBATIM_STRING_5: &str = "string";
pub const VERBATIM__CHAT_COMPLETIONS_6: &str = "/chat/completions";
pub const VERBATIM_OPENROUTER_API_KEY_7: &str = "OPENROUTER_API_KEY";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openrouter".to_string(),
        value: Value::Null,
    }
}
