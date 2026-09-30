// source: src/cli/cmd/run/theme.ts — exports: [RunEntryTheme, RunSplashTheme, RunFooterTheme, RunBlockTheme, RunTheme, transparent, resolveTheme, generateSystem, RUN_THEME_FALLBACK, resolveRunTheme]
// PROVISIONAL pending crates/plugin: `@opencode-ai/plugin/tui`
/// verbatim strings (source order, quoted for V2 audit):
/// - "thinkingOpacity"
/// - "selectedListItemText"
/// - "backgroundMenu"
/// - "transparent"
/// - " -> "
/// - "${value}"
/// - "@opencode-ai/tui/context/theme"
/// source: `export type RunEntryTheme` — shape as JSON value; CI verifies.
pub type RunEntryTheme = serde_json::Value;
/// source: `export type RunSplashTheme` — shape as JSON value; CI verifies.
pub type RunSplashTheme = serde_json::Value;
/// source: `export type RunFooterTheme` — shape as JSON value; CI verifies.
pub type RunFooterTheme = serde_json::Value;
/// source: `export type RunBlockTheme` — shape as JSON value; CI verifies.
pub type RunBlockTheme = serde_json::Value;
/// source: `export type RunTheme` — shape as JSON value; CI verifies.
pub type RunTheme = serde_json::Value;
/// source: `export const transparent` — shape as JSON value; CI verifies.
pub type transparent = serde_json::Value;
/// source: `export function resolveTheme` — stub shell; CI verifies behavior.
pub fn resolveTheme(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function generateSystem` — stub shell; CI verifies behavior.
pub fn generateSystem(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const RUN_THEME_FALLBACK` — shape as JSON value; CI verifies.
pub type RUN_THEME_FALLBACK = serde_json::Value;
/// source: `export function resolveRunTheme` — stub shell; CI verifies behavior.
pub fn resolveRunTheme(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
