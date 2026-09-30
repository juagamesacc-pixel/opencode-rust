// source: src/cli/cmd/run/runtime.ts — exports: [RunRuntimeDeps, runInteractiveLocalMode, runInteractiveMode, pickVariant, resolveVariant, runPromptQueue]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
// PROVISIONAL pending crates/core: `@opencode-ai/core/flag/flag`
/// verbatim strings (source order, quoted for V2 audit):
/// - "directory"
/// - "sessionID"
/// - "sessionTitle"
/// - "./stream.transport"
/// - "createSessionTransport"
/// - "formatUnknownError"
/// - "Failed to create session"
/// - "send.permission.reply"
/// source: `export type RunRuntimeDeps` — shape as JSON value; CI verifies.
pub type RunRuntimeDeps = serde_json::Value;
/// source: `export function runInteractiveLocalMode` — stub shell; CI verifies behavior.
pub fn runInteractiveLocalMode(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function runInteractiveMode` — stub shell; CI verifies behavior.
pub fn runInteractiveMode(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
// source: `export { pickVariant }` — re-export; resolve via crate path.
// source: `export { resolveVariant }` — re-export; resolve via crate path.
// source: `export { runPromptQueue }` — re-export; resolve via crate path.
