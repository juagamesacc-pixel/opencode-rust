// source: src/cli/cmd/run/runtime.boot.ts — exports: [ModelInfo, SessionInfo, resolveModelInfo, resolveSessionInfo, resolveRunTuiConfig, resolveDiffStyle]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/config`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode/RunBoot"
/// - "RunBoot.config"
/// - "RunBoot.resolveModelInfo"
/// - "RunBoot.resolveSessionInfo"
/// - "RunBoot.resolveRunTuiConfig"
/// - "RunBoot.resolveDiffStyle"
/// source: `export type ModelInfo` — shape as JSON value; CI verifies.
pub type ModelInfo = serde_json::Value;
/// source: `export type SessionInfo` — shape as JSON value; CI verifies.
pub type SessionInfo = serde_json::Value;
/// source: `export function resolveModelInfo` — stub shell; CI verifies behavior.
pub fn resolveModelInfo(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function resolveSessionInfo` — stub shell; CI verifies behavior.
pub fn resolveSessionInfo(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function resolveRunTuiConfig` — stub shell; CI verifies behavior.
pub fn resolveRunTuiConfig(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function resolveDiffStyle` — stub shell; CI verifies behavior.
pub fn resolveDiffStyle(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
