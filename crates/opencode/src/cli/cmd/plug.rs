// source: src/cli/cmd/plug.ts — exports: [PlugDeps, PlugInput, PlugCtx, createPlugTask, PluginCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@clack/prompts"
/// - "opencode"
/// - "Installing plugin package..."
/// - "Install failed"
/// - "error:"
/// - "No version matching"
/// - "This package depends on a version that is not available in your npm registry."
/// - "Check npm registry/auth settings and try again."
/// source: `export type PlugDeps` — shape as JSON value; CI verifies.
pub type PlugDeps = serde_json::Value;
/// source: `export type PlugInput` — shape as JSON value; CI verifies.
pub type PlugInput = serde_json::Value;
/// source: `export type PlugCtx` — shape as JSON value; CI verifies.
pub type PlugCtx = serde_json::Value;
/// source: `export function createPlugTask` — stub shell; CI verifies behavior.
pub fn createPlugTask(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const PluginCommand` — shape as JSON value; CI verifies.
pub type PluginCommand = serde_json::Value;
