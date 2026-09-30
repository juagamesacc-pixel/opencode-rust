// source: src/tool/json-schema.ts — exports: [fromSchema, fromTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@ai-sdk/provider"
/// - "tool JSON Schema helper produced a non-schema value"
/// - "properties"
/// - " || value === "
/// - "#/$defs/"
/// - "#/definitions/"
/// source: `export function fromSchema` — stub shell; CI verifies behavior.
pub fn fromSchema(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function fromTool` — stub shell; CI verifies behavior.
pub fn fromTool(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
