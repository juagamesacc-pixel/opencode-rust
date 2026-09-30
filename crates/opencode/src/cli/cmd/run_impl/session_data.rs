// source: src/cli/cmd/run/session-data.ts — exports: [SessionData, SessionDataInput, SessionDataOutput, createSessionData, formatError, pickBlockerView, blockerStatus, bootstrapSessionData, flushInterrupted, reduceSessionData]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
/// verbatim strings (source order, quoted for V2 audit):
/// - "assistant"
/// - "en-US"
/// - "currency"
/// - "reasoning"
/// - "unknown error"
/// - "MessageAbortedError"
/// - "permission"
/// - "question"
/// source: `export type SessionData` — shape as JSON value; CI verifies.
pub type SessionData = serde_json::Value;
/// source: `export type SessionDataInput` — shape as JSON value; CI verifies.
pub type SessionDataInput = serde_json::Value;
/// source: `export type SessionDataOutput` — shape as JSON value; CI verifies.
pub type SessionDataOutput = serde_json::Value;
/// source: `export function createSessionData` — stub shell; CI verifies behavior.
pub fn createSessionData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function formatError` — stub shell; CI verifies behavior.
pub fn formatError(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function pickBlockerView` — stub shell; CI verifies behavior.
pub fn pickBlockerView(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function blockerStatus` — stub shell; CI verifies behavior.
pub fn blockerStatus(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function bootstrapSessionData` — stub shell; CI verifies behavior.
pub fn bootstrapSessionData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function flushInterrupted` — stub shell; CI verifies behavior.
pub fn flushInterrupted(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function reduceSessionData` — stub shell; CI verifies behavior.
pub fn reduceSessionData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
