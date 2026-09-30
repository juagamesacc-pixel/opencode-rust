// source: src/tool/grep.ts — exports: [Parameters, GrepTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending crates/core: `@opencode-ai/core/ripgrep`
/// verbatim strings (source order, quoted for V2 audit):
/// - "The regex pattern to search for in file contents"
/// - "The directory to search in. Defaults to the current working directory."
/// - "File pattern to include in the search (e.g. \"*.js\", \"*.{ts,tsx}\")"
/// - "No files found"
/// - "pattern is required"
/// - "Directory"
/// - "directory"
/// - " (more matches available)"
/// source: `src/tool/grep.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"grep.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const GrepTool` — shape as JSON value; CI verifies.
pub type GrepTool = serde_json::Value;
