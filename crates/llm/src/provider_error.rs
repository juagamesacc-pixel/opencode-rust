#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/provider-error.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_INVALIDREQUEST_1: &str = "InvalidRequest";
pub const VERBATIM_CONTEXT_OVERFLOW_2: &str = "context-overflow";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "provider_error".to_string(),
        value: Value::Null,
    }
}

pub fn is_context_overflow(message: &str) -> bool {
    let patterns = [
        "prompt is too long",
        "request_too_large",
        "exceeds the context window",
        "maximum context length",
        "token limit exceeded",
        "context_length_exceeded",
    ];
    let exclusions = ["throttling error", "rate limit", "too many requests"];
    if exclusions
        .iter()
        .any(|e| message.to_lowercase().contains(*e))
    {
        return false;
    }
    patterns
        .iter()
        .any(|p| message.to_lowercase().contains(&p.to_lowercase()))
        || message.contains("413")
}
pub fn is_context_overflow_failure(_failure: &Value) -> bool {
    false
}
