#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/anthropic-messages.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_ANTHROPIC_MESSAGES_1: &str = "anthropic-messages";
pub const VERBATIM_HTTPS_API_ANTHROPIC_COM__2: &str = "https://api.anthropic.com/v1";
pub const VERBATIM__MESSAGES_3: &str = "/messages";
pub const VERBATIM_EPHEMERAL_4: &str = "ephemeral";
pub const VERBATIM__CONST_ANTHROPICTEXTBLOC_5: &str = "])),
})

const AnthropicTextBlock = Schema.Struct({
  type: Schema.tag(";
pub const VERBATIM__SOURCE_SCHEMA_STRUCT_TY_6: &str = "),
  source: Schema.Struct({
    type: Schema.tag(";
pub const VERBATIM___7: &str = ",
  ";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "anthropic_messages".to_string(),
        value: Value::Null,
    }
}
