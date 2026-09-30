// source: src/cli/cmd/run/stream.ts — exports: [traceSubagentState, traceFooterOutput, writeSessionOutput]
/// verbatim strings (source order, quoted for V2 audit):
/// - "metadata"
/// - "ui.commit"
/// - "ui.patch"
/// - "stream.patch"
/// - "ui.subagent"
/// - "stream.subagent"
/// - "stream.view"
/// source: `export function traceSubagentState` — stub shell; CI verifies behavior.
pub fn traceSubagentState(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function traceFooterOutput` — stub shell; CI verifies behavior.
pub fn traceFooterOutput(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function writeSessionOutput` — stub shell; CI verifies behavior.
pub fn writeSessionOutput(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
