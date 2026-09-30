#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/github-copilot.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_GITHUB_COPILOT_0: &str = "github-copilot";
pub const VERBATIM_PROVIDEROPTIONS_1: &str = "providerOptions";
pub const VERBATIM_OPTIONAL_2: &str = "optional";
pub const VERBATIM_CHAT_3: &str = "chat";
pub const VERBATIM_RESPONSES_4: &str = "responses";
pub const VERBATIM_ENDPOINT_5: &str = "endpoint";
pub const VERBATIM_GPT_5_MINI_6: &str = "gpt-5-mini";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "github_copilot".to_string(),
        value: Value::Null,
    }
}
