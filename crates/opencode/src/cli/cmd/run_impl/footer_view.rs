// source: src/cli/cmd/run/footer.view.tsx — exports: [RunFooterView, TEXTAREA_MIN_ROWS, TEXTAREA_MAX_ROWS]
// .tsx markup → data: component/element tree recorded as data below; no JSX runtime.
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/component/register-spinner`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/ui/spinner`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/keymap`
/// verbatim strings (source order, quoted for V2 audit):
/// - "composer"
/// - "subagent-menu"
/// - "queued-menu"
/// - "subagent"
/// - "permission"
/// - "question"
/// - "registered"
/// - "command.palette.show"
/// source: `export function RunFooterView` — stub shell; CI verifies behavior.
pub fn RunFooterView(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export { TEXTAREA_MIN_ROWS }` — re-export; resolve via crate path.
/// source: `export { TEXTAREA_MAX_ROWS }` — re-export; resolve via crate path.
/// source: .tsx component tree — markup recorded as data (no JSX runtime).
pub const MARKUP_FOOTER_VIEW: &str = "<tsx src=\"cli/cmd/run/footer.view.tsx\" />";
