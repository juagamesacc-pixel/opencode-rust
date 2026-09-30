// source: src/tool/apply_patch.ts — exports: [Parameters, ApplyPatchTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem/watcher`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem`
/// verbatim strings (source order, quoted for V2 audit):
/// - "The full patch text that describes all changes to be made"
/// - "apply_patch"
/// - "ApplyPatchTool.execute"
/// - "patchText is required"
/// - ").replace(/\\r/g, "
/// - "*** Begin Patch\\n*** End Patch"
/// - "patch rejected: empty patch"
/// - "apply_patch verification failed: no hunks found"
/// source: `src/tool/apply_patch.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"apply_patch.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const ApplyPatchTool` — shape as JSON value; CI verifies.
pub type ApplyPatchTool = serde_json::Value;
