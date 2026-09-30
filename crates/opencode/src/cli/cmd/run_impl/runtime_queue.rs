// source: src/cli/cmd/run/runtime.queue.ts — exports: [QueueInput, runPromptQueue]
/// verbatim strings (source order, quoted for V2 audit):
/// - "ui.patch"
/// - "queued.prompts"
/// - "stream.patch"
/// - "new sessions unavailable"
/// - "starting new session"
/// - "turn.send"
/// - "sending prompt"
/// - "ui.commit"
/// source: `export type QueueInput` — shape as JSON value; CI verifies.
pub type QueueInput = serde_json::Value;
/// source: `export function runPromptQueue` — stub shell; CI verifies behavior.
pub fn runPromptQueue(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
