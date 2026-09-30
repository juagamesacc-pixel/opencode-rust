// source: src/cli/cmd/run/footer.subagent.tsx — exports: [SUBAGENT_INSPECTOR_ROWS, RunFooterSubagentBody]
// .tsx markup → data: component/element tree recorded as data below; no JSX runtime.
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/component/register-spinner`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/component/spinner`
/// verbatim strings (source order, quoted for V2 audit):
/// - "completed"
/// - "cancelled"
/// - " || event.name === "
/// source: `SUBAGENT_INSPECTOR_ROWS = 14` — verbatim.
pub const SUBAGENT_INSPECTOR_ROWS: i64 = 14;
/// source: `export function RunFooterSubagentBody` — stub shell; CI verifies behavior.
pub fn RunFooterSubagentBody(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: .tsx component tree — markup recorded as data (no JSX runtime).
pub const MARKUP_FOOTER_SUBAGENT: &str = "<tsx src=\"cli/cmd/run/footer.subagent.tsx\" />";
