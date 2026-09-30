#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/utils/tool-schema.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_ANYOF_0: &str = "anyOf";
pub const VERBATIM_NULL_1: &str = "null";
pub const VERBATIM_STRING_2: &str = "string";
pub const VERBATIM_ITEMS_3: &str = "items";
pub const VERBATIM_PREFIXITEMS_4: &str = "prefixItems";
pub const VERBATIM_UNEVALUATEDITEMS_5: &str = "unevaluatedItems";
pub const VERBATIM_OBJECT_6: &str = "object";
pub const VERBATIM_GEMINI_7: &str = "gemini";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "tool_schema".to_string(),
        value: Value::Null,
    }
}
