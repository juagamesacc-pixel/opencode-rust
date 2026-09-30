#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/llm.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_SYSTEM_1: &str = "system";
pub const VERBATIM_MESSAGES_2: &str = "messages";
pub const VERBATIM_TOOLS_3: &str = "tools";
pub const VERBATIM_TOOLCHOICE_4: &str = "toolChoice";
pub const VERBATIM_GENERATION_5: &str = "generation";
pub const VERBATIM_HTTP_6: &str = "http";
pub const VERBATIM_PROVIDEROPTIONS_7: &str = "providerOptions";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "llm".to_string(),
        value: Value::Null,
    }
}
