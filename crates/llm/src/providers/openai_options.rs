#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/openai-options.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_AUTO_0: &str = "auto";
pub const VERBATIM_GPT_5_1: &str = "gpt-5";
pub const VERBATIM_GPT_5_CHAT_2: &str = "gpt-5-chat";
pub const VERBATIM_GPT_5_PRO_3: &str = "gpt-5-pro";
pub const VERBATIM_MEDIUM_4: &str = "medium";
pub const VERBATIM_REASONING_ENCRYPTED_CONT_5: &str = "reasoning.encrypted_content";
pub const VERBATIM_GPT_5__6: &str = "gpt-5.";
pub const VERBATIM_CODEX_7: &str = "codex";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openai_options".to_string(),
        value: Value::Null,
    }
}
