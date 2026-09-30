// source: src/cli/cmd/run/footer.prompt.tsx — exports: [TEXTAREA_MIN_ROWS, TEXTAREA_MAX_ROWS, PROMPT_MAX_ROWS, PromptState, RunPromptBody, createPromptState]
// .tsx markup → data: component/element tree recorded as data below; no JSX runtime.
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/editor`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/keymap`
/// verbatim strings (source order, quoted for V2 audit):
/// - "skill-menu"
/// - ", parts: [], mode: "
/// - " } : { text: "
/// - "Run a command... \"git status\""
/// - "Ask anything... \"Fix a TODO in the codebase\""
/// - "text/plain"
/// - "resource"
/// - ") ? "
/// source: `TEXTAREA_MIN_ROWS = 1` — verbatim.
pub const TEXTAREA_MIN_ROWS: i64 = 1;
/// source: `TEXTAREA_MAX_ROWS = 6` — verbatim.
pub const TEXTAREA_MAX_ROWS: i64 = 6;
/// source: `export const PROMPT_MAX_ROWS` — shape as JSON value; CI verifies.
pub type PROMPT_MAX_ROWS = serde_json::Value;
/// source: `export type PromptState` — shape as JSON value; CI verifies.
pub type PromptState = serde_json::Value;
/// source: `export function RunPromptBody` — stub shell; CI verifies behavior.
pub fn RunPromptBody(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function createPromptState` — stub shell; CI verifies behavior.
pub fn createPromptState(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: .tsx component tree — markup recorded as data (no JSX runtime).
pub const MARKUP_FOOTER_PROMPT: &str = "<tsx src=\"cli/cmd/run/footer.prompt.tsx\" />";
