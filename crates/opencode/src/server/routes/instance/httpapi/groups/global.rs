// source: src/server/routes/instance/httpapi/groups/global.ts — exports: [GlobalUpgradeInput, GlobalPaths, GlobalApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/config`
// PROVISIONAL pending crates/core: `@opencode-ai/core/event`
// PROVISIONAL pending crates/core: `@opencode-ai/core/account`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/config/config"
/// - "GlobalEvent"
/// - "Expected a semantic version"
/// - "/global/health"
/// - "/global/event"
/// - "/global/config"
/// - "/global/dispose"
/// - "/global/upgrade"
/// source: `export const GlobalUpgradeInput` — shape as JSON value; CI verifies.
pub type GlobalUpgradeInput = serde_json::Value;
/// source: `export const GlobalPaths` — shape as JSON value; CI verifies.
pub type GlobalPaths = serde_json::Value;
/// source: `export const GlobalApi` — shape as JSON value; CI verifies.
pub type GlobalApi = serde_json::Value;
