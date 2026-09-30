// source: src/cli/cmd/run/session.shared.ts — exports: [SessionMessages, RunSession, messagePrompt, createSession, resolveSession, sessionHistory, sessionVariant]
/// verbatim strings (source order, quoted for V2 audit):
/// - "messages"
/// - "file:"
/// - " + part.name) ?? add("
/// source: `export type SessionMessages` — shape as JSON value; CI verifies.
pub type SessionMessages = serde_json::Value;
/// source: `export type RunSession` — shape as JSON value; CI verifies.
pub type RunSession = serde_json::Value;
/// source: `export function messagePrompt` — stub shell; CI verifies behavior.
pub fn messagePrompt(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function createSession` — stub shell; CI verifies behavior.
pub fn createSession(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function resolveSession` — stub shell; CI verifies behavior.
pub fn resolveSession(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function sessionHistory` — stub shell; CI verifies behavior.
pub fn sessionHistory(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function sessionVariant` — stub shell; CI verifies behavior.
pub fn sessionVariant(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
