#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/route/client.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_EFFECT_OPTION_1: &str = "effect/Option";
pub const VERBATIM_PROVIDER_2: &str = "provider";
pub const VERBATIM_ROUTE_3: &str = "route";
pub const VERBATIM_STRING_4: &str = "string";
pub const VERBATIM_DEFAULTS_5: &str = "defaults";
pub const VERBATIM_TRANSPORT_6: &str = "transport";
pub const VERBATIM_LLM_COMPILE_7: &str = "LLM.compile";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "client".to_string(),
        value: Value::Null,
    }
}
pub const LLM_CLIENT_SERVICE_ID: &str = "@opencode-ai/llm/LLMClient";
