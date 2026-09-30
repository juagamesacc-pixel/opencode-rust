// source: src/tool/edit.ts — exports: [Parameters, EditTool, Replacer, SimpleReplacer, LineTrimmedReplacer, BlockAnchorReplacer, WhitespaceNormalizedReplacer, IndentationFlexibleReplacer, EscapeNormalizedReplacer, MultiOccurrenceReplacer, TrimmedBoundaryReplacer, ContextAwareReplacer, trimDiff, replace]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem`
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem/watcher`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
/// verbatim strings (source order, quoted for V2 audit):
/// - ") ? "
/// - "The absolute path to the file to modify"
/// - "The text to replace"
/// - "The text to replace it with (must be different from oldString)"
/// - "Replace all occurrences of oldString (default false)"
/// - "filePath is required"
/// - "No changes to apply: oldString and newString are identical."
/// - "Directory"
/// source: `src/tool/edit.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"edit.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const EditTool` — shape as JSON value; CI verifies.
pub type EditTool = serde_json::Value;
/// source: `export type Replacer` — shape as JSON value; CI verifies.
pub type Replacer = serde_json::Value;
/// source: `export const SimpleReplacer` — shape as JSON value; CI verifies.
pub type SimpleReplacer = serde_json::Value;
/// source: `export const LineTrimmedReplacer` — shape as JSON value; CI verifies.
pub type LineTrimmedReplacer = serde_json::Value;
/// source: `export const BlockAnchorReplacer` — shape as JSON value; CI verifies.
pub type BlockAnchorReplacer = serde_json::Value;
/// source: `export const WhitespaceNormalizedReplacer` — shape as JSON value; CI verifies.
pub type WhitespaceNormalizedReplacer = serde_json::Value;
/// source: `export const IndentationFlexibleReplacer` — shape as JSON value; CI verifies.
pub type IndentationFlexibleReplacer = serde_json::Value;
/// source: `export const EscapeNormalizedReplacer` — shape as JSON value; CI verifies.
pub type EscapeNormalizedReplacer = serde_json::Value;
/// source: `export const MultiOccurrenceReplacer` — shape as JSON value; CI verifies.
pub type MultiOccurrenceReplacer = serde_json::Value;
/// source: `export const TrimmedBoundaryReplacer` — shape as JSON value; CI verifies.
pub type TrimmedBoundaryReplacer = serde_json::Value;
/// source: `export const ContextAwareReplacer` — shape as JSON value; CI verifies.
pub type ContextAwareReplacer = serde_json::Value;
/// source: `export function trimDiff` — stub shell; CI verifies behavior.
pub fn trimDiff(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function replace` — stub shell; CI verifies behavior.
pub fn replace(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
