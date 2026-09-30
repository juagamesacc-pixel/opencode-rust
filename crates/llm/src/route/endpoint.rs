#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/route/endpoint.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_PATH_0: &str = "path";
pub const VERBATIM_FUNCTION_1: &str = "function";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "endpoint".to_string(),
        value: Value::Null,
    }
}
pub const ENDPOINT_DEFAULT_QUERY: &str = "";
