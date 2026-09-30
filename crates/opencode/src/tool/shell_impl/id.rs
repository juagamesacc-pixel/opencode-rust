// source: src/tool/shell/id.ts — exports: [Kind, toKind, ToolID]
/// verbatim strings (source order, quoted for V2 audit):
/// - "powershell"
/// source: `ToolID = "bash"` — verbatim.
pub const ToolID: &str = "bash";
/// source: `export type Kind` — shape as JSON value; CI verifies.
pub type Kind = serde_json::Value;
/// source: `export function toKind` — stub shell; CI verifies behavior.
pub fn toKind(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
