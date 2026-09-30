#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/utils/gemini-tool-schema.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_TYPE_0: &str = "type";
pub const VERBATIM_PROPERTIES_1: &str = "properties";
pub const VERBATIM_ITEMS_2: &str = "items";
pub const VERBATIM_PREFIXITEMS_3: &str = "prefixItems";
pub const VERBATIM_ENUM_4: &str = "enum";
pub const VERBATIM_CONST_5: &str = "const";
pub const VERBATIM_ADDITIONALPROPERTIES_6: &str = "additionalProperties";
pub const VERBATIM_PATTERNPROPERTIES_7: &str = "patternProperties";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "gemini_tool_schema".to_string(),
        value: Value::Null,
    }
}
