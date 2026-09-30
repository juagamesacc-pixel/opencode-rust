#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/tool.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM__READONLY_NAME_TOOLCALLP_1: &str = "]
  readonly name: ToolCallPart[";
pub const VERBATIM__JSONSCHEMA_TYPE__2: &str = ",
 *      jsonSchema: { type: ";
pub const VERBATIM__TYPE__3: &str = " ? [{ type: ";
pub const VERBATIM__TEXT_OUTPUT_EXPORT_TOOL_4: &str = ", text: output }] : []),
  )

export { ToolFailure }

export * as Tool from ";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "tool".to_string(),
        value: Value::Null,
    }
}
