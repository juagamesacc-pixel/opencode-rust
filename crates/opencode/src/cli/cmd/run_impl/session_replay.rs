// source: src/cli/cmd/run/session-replay.ts — exports: [SessionReplay, replaySession, replayLocalRows, replayActiveText]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/sdk/v2"
/// - "The following tool was executed by the user"
/// - "assistant"
/// - "message.updated"
/// - "message.part.updated"
/// - "progress"
/// source: `export type SessionReplay` — shape as JSON value; CI verifies.
pub type SessionReplay = serde_json::Value;
/// source: `export function replaySession` — stub shell; CI verifies behavior.
pub fn replaySession(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function replayLocalRows` — stub shell; CI verifies behavior.
pub fn replayLocalRows(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function replayActiveText` — stub shell; CI verifies behavior.
pub fn replayActiveText(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
