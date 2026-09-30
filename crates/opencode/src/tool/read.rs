// source: src/tool/read.ts — exports: [Parameters, ReadTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
/// verbatim strings (source order, quoted for V2 audit):
/// - "image/jpeg"
/// - "image/png"
/// - "image/gif"
/// - "image/webp"
/// - "ReadStop"
/// - "The absolute path to the file or directory to read"
/// - "The line number to start reading from (1-indexed)"
/// - "The maximum number of lines to read (defaults to 2000)"
/// source: `src/tool/read.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"read.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const ReadTool` — shape as JSON value; CI verifies.
pub type ReadTool = serde_json::Value;
