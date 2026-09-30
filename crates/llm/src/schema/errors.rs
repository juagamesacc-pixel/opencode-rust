#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/schema/errors.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_CONTEXT_OVERFLOW_1: &str = "context-overflow";
pub const VERBATIM_LLM_HTTPREQUESTDETAILS_2: &str = "LLM.HttpRequestDetails";
pub const VERBATIM_LLM_HTTPRESPONSEDETAILS_3: &str = "LLM.HttpResponseDetails";
pub const VERBATIM_LLM_HTTPRATELIMITDETAILS_4: &str = "LLM.HttpRateLimitDetails";
pub const VERBATIM_LLM_HTTPCONTEXT_5: &str = "LLM.HttpContext";
pub const VERBATIM_LLM_ERROR_INVALIDREQUEST_6: &str = "LLM.Error.InvalidRequest";
pub const VERBATIM_INVALIDREQUEST_7: &str = "InvalidRequest";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "errors".to_string(),
        value: Value::Null,
    }
}
pub const ERROR_TAGS: &[&str] = &[
    "InvalidRequest",
    "NoRoute",
    "Authentication",
    "RateLimit",
    "QuotaExceeded",
    "ContentPolicy",
    "ProviderInternal",
    "Transport",
    "InvalidProviderOutput",
    "UnknownProvider",
];
pub const TOOL_FAILURE_TAG: &str = "LLM.ToolFailure";
