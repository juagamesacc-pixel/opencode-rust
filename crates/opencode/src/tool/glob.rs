// source: src/tool/glob.ts — exports: [Parameters, GlobTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending crates/core: `@opencode-ai/core/ripgrep`
/// verbatim strings (source order, quoted for V2 audit):
/// - "The glob pattern to match files against"
/// - "undefined"
/// - "directory"
/// - "No files found"
/// source: `src/tool/glob.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"glob.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const GlobTool` — shape as JSON value; CI verifies.
pub type GlobTool = serde_json::Value;
