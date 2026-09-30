// source: src/tool/lsp.ts — exports: [Parameters, LspTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
/// verbatim strings (source order, quoted for V2 audit):
/// - "goToDefinition"
/// - "findReferences"
/// - "documentSymbol"
/// - "workspaceSymbol"
/// - "goToImplementation"
/// - "prepareCallHierarchy"
/// - "incomingCalls"
/// - "outgoingCalls"
/// source: `src/tool/lsp.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"lsp.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const LspTool` — shape as JSON value; CI verifies.
pub type LspTool = serde_json::Value;
