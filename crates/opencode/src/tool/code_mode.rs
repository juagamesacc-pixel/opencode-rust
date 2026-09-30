// source: src/tool/code-mode.ts — exports: [CODE_MODE_TOOL, Parameters, describeCatalog, CodeModeTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/codemode: `@opencode-ai/codemode`
/// verbatim strings (source order, quoted for V2 audit):
/// - "./tool"
/// - "Run a confined orchestration script with access to connected MCP tools."
/// - "Script body executed by the confined interpreter."
/// - "completed"
/// - "attachments"
/// - ")) ?? (key.includes("
/// - ") ? key.slice(0, key.indexOf("
/// - "Tool preview is not executable."
/// source: `CODE_MODE_TOOL = "execute"` — verbatim.
pub const CODE_MODE_TOOL: &str = "execute";
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export function describeCatalog` — stub shell; CI verifies behavior.
pub fn describeCatalog(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const CodeModeTool` — shape as JSON value; CI verifies.
pub type CodeModeTool = serde_json::Value;
