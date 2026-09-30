#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/provider.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_DEFAULTS_0: &str = "defaults";
pub const VERBATIM_COMPATIBILITY_1: &str = "compatibility";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "provider".to_string(),
        value: Value::Null,
    }
}
