// source: src/tool/shell/prompt.ts — exports: [Limits, parameterSchema, Parameters, render]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
/// verbatim strings (source order, quoted for V2 audit):
/// - "powershell"
/// - "The command to execute"
/// - "Optional timeout in milliseconds"
/// - "PowerShell (7+)"
/// - "Windows PowerShell (5.1)"
/// - ") return "
/// - "Hello $name"
/// - "path/to/exe"
/// source: `export type Limits` — shape as JSON value; CI verifies.
pub type Limits = serde_json::Value;
/// source: `export function parameterSchema` — stub shell; CI verifies behavior.
pub fn parameterSchema(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export function render` — stub shell; CI verifies behavior.
pub fn render(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
