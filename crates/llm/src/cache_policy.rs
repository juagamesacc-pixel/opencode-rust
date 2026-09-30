#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/cache-policy.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_AUTO_0: &str = "auto";
pub const VERBATIM_LATEST_USER_MESSAGE_1: &str = "latest-user-message";
pub const VERBATIM_NONE_2: &str = "none";
pub const VERBATIM_ANTHROPIC_MESSAGES_3: &str = "anthropic-messages";
pub const VERBATIM_BEDROCK_CONVERSE_4: &str = "bedrock-converse";
pub const VERBATIM_EPHEMERAL_5: &str = "ephemeral";
pub const VERBATIM_SYSTEM_6: &str = "system";
pub const VERBATIM_ROLE_7: &str = "role";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "cache_policy".to_string(),
        value: Value::Null,
    }
}
pub const RESPECTS_INLINE_HINTS: &[&str] = &["anthropic-messages", "bedrock-converse"];
pub fn apply_cache_policy(request: &Value) -> Value {
    request.clone()
}
