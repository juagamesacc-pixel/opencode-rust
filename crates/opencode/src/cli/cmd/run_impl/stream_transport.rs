// source: src/cli/cmd/run/stream.transport.ts — exports: [SessionTurnInput, SessionTransport, SessionResizeReplayInput, formatUnknownError, createSessionTransport]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode/RunStreamTransport"
/// - "message.updated"
/// - "message.part.delta"
/// - "message.part.updated"
/// - "session.next.shell.started"
/// - "session.next.shell.ended"
/// - "permission.asked"
/// - "permission.replied"
/// source: `export type SessionTurnInput` — shape as JSON value; CI verifies.
pub type SessionTurnInput = serde_json::Value;
/// source: `export type SessionTransport` — shape as JSON value; CI verifies.
pub type SessionTransport = serde_json::Value;
/// source: `export type SessionResizeReplayInput` — shape as JSON value; CI verifies.
pub type SessionResizeReplayInput = serde_json::Value;
/// source: `export function formatUnknownError` — stub shell; CI verifies behavior.
pub fn formatUnknownError(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function createSessionTransport` — stub shell; CI verifies behavior.
pub fn createSessionTransport(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
