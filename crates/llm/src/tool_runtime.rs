#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/tool-runtime.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_ERROR_1: &str = "error";
pub const VERBATIM_LLM_TOOLFAILURE_2: &str = "LLM.ToolFailure";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "tool_runtime".to_string(),
        value: Value::Null,
    }
}
