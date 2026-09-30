// source: src/tool/write.ts — exports: [Parameters, WriteTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem`
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem/watcher`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
/// verbatim strings (source order, quoted for V2 audit):
/// - "The content to write to the file"
/// - "The absolute path to the file to write (must be absolute, not relative)"
/// - "Wrote file successfully."
/// - "document"
/// source: `src/tool/write.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"write.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const WriteTool` — shape as JSON value; CI verifies.
pub type WriteTool = serde_json::Value;
