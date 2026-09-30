// source: src/tool/shell.ts — exports: [ShellTool, Parameters]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `node:fs` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending crates/core: `@opencode-ai/core/shell`
// PROVISIONAL pending external `effect/unstable/process` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/process/ChildProcessSpawner` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "push-location"
/// - "set-location"
/// - "get-content"
/// - "set-content"
/// - "add-content"
/// - "copy-item"
/// - "move-item"
/// - "remove-item"
/// source: `export const ShellTool` — shape as JSON value; CI verifies.
pub type ShellTool = serde_json::Value;
// source: `export { Parameters }` — re-export; resolve via crate path.
