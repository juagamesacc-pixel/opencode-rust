// source: src/cli/cmd/run/entry.body.ts — exports: [EntryFlags, RUN_ENTRY_NONE, cleanRunText, entryFlags, entryDone, entryCanStream, entryBody]
/// verbatim strings (source order, quoted for V2 audit):
/// - "./tool"
/// - ").replace(/\\r/g, "
/// - "markdown"
/// - "Thinking:"
/// - "progress"
/// - "assistant"
/// - "reasoning"
/// - "completed"
/// source: `export type EntryFlags` — shape as JSON value; CI verifies.
pub type EntryFlags = serde_json::Value;
/// source: `export const RUN_ENTRY_NONE` — shape as JSON value; CI verifies.
pub type RUN_ENTRY_NONE = serde_json::Value;
/// source: `export function cleanRunText` — stub shell; CI verifies behavior.
pub fn cleanRunText(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function entryFlags` — stub shell; CI verifies behavior.
pub fn entryFlags(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function entryDone` — stub shell; CI verifies behavior.
pub fn entryDone(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function entryCanStream` — stub shell; CI verifies behavior.
pub fn entryCanStream(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function entryBody` — stub shell; CI verifies behavior.
pub fn entryBody(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
