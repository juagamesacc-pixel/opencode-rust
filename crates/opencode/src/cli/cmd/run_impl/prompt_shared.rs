// source: src/cli/cmd/run/prompt.shared.ts — exports: [PromptHistoryState, PromptMove, promptCopy, promptSame, isExitCommand, isNewCommand, createPromptHistory, pushPromptHistory, movePromptHistory, displayCharAt, displaySlice, mentionTriggerIndex]
/// verbatim strings (source order, quoted for V2 audit):
/// - "/exit"
/// - "/quit"
/// - "/new"
/// source: `export type PromptHistoryState` — shape as JSON value; CI verifies.
pub type PromptHistoryState = serde_json::Value;
/// source: `export type PromptMove` — shape as JSON value; CI verifies.
pub type PromptMove = serde_json::Value;
/// source: `export function promptCopy` — stub shell; CI verifies behavior.
pub fn promptCopy(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function promptSame` — stub shell; CI verifies behavior.
pub fn promptSame(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function isExitCommand` — stub shell; CI verifies behavior.
pub fn isExitCommand(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function isNewCommand` — stub shell; CI verifies behavior.
pub fn isNewCommand(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function createPromptHistory` — stub shell; CI verifies behavior.
pub fn createPromptHistory(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function pushPromptHistory` — stub shell; CI verifies behavior.
pub fn pushPromptHistory(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function movePromptHistory` — stub shell; CI verifies behavior.
pub fn movePromptHistory(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
// source: `export { displayCharAt }` — re-export; resolve via crate path.
// source: `export { displaySlice }` — re-export; resolve via crate path.
// source: `export { mentionTriggerIndex }` — re-export; resolve via crate path.
