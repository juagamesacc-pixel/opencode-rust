// source: src/server/shared/tui-control.ts — exports: [TuiRequest, nextTuiRequest, submitTuiRequest, submitTuiResponse, nextTuiResponse]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/util/queue"
/// source: `export const TuiRequest` — shape as JSON value; CI verifies.
pub type TuiRequest = serde_json::Value;
/// source: `export function nextTuiRequest` — stub shell; CI verifies behavior.
pub fn nextTuiRequest(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function submitTuiRequest` — stub shell; CI verifies behavior.
pub fn submitTuiRequest(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function submitTuiResponse` — stub shell; CI verifies behavior.
pub fn submitTuiResponse(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function nextTuiResponse` — stub shell; CI verifies behavior.
pub fn nextTuiResponse(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
