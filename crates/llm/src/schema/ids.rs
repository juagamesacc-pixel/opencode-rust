#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/schema/ids.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_LLM_MODELID_1: &str = "LLM.ModelID";
pub const VERBATIM_LLM_PROVIDERID_2: &str = "LLM.ProviderID";
pub const VERBATIM_NONE_3: &str = "none";
pub const VERBATIM_MINIMAL_4: &str = "minimal";
pub const VERBATIM_LOW_5: &str = "low";
pub const VERBATIM_MEDIUM_6: &str = "medium";
pub const VERBATIM_HIGH_7: &str = "high";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "ids".to_string(),
        value: Value::Null,
    }
}
pub const REASONING_EFFORTS: &[&str] =
    &["none", "minimal", "low", "medium", "high", "xhigh", "max"];
pub const FINISH_REASONS: &[&str] = &[
    "stop",
    "length",
    "tool-calls",
    "content-filter",
    "error",
    "unknown",
];
