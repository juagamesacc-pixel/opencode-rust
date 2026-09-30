// source: src/server/routes/instance/httpapi/api.ts — exports: [ServerApi, RootHttpApi, InstanceHttpApi, OpenCodeHttpApi, RootHttpApiType, InstanceHttpApiType]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/event`
// PROVISIONAL pending crates/core: `@opencode-ai/core/credential`
// PROVISIONAL pending crates/core: `@opencode-ai/core/integration`
// PROVISIONAL pending crates/core: `@opencode-ai/core/skill`
// PROVISIONAL pending crates/protocol: `@opencode-ai/protocol/api`
// PROVISIONAL pending crates/server: `@opencode-ai/server/location`
// PROVISIONAL pending crates/server: `@opencode-ai/server/middleware/session-location`
/// verbatim strings (source order, quoted for V2 audit):
/// - "opencode-root"
/// - "opencode-instance"
/// - "opencode"
/// source: `export const ServerApi` — shape as JSON value; CI verifies.
pub type ServerApi = serde_json::Value;
/// source: `export const RootHttpApi` — shape as JSON value; CI verifies.
pub type RootHttpApi = serde_json::Value;
/// source: `export const InstanceHttpApi` — shape as JSON value; CI verifies.
pub type InstanceHttpApi = serde_json::Value;
/// source: `export const OpenCodeHttpApi` — shape as JSON value; CI verifies.
pub type OpenCodeHttpApi = serde_json::Value;
/// source: `export type RootHttpApiType` — shape as JSON value; CI verifies.
pub type RootHttpApiType = serde_json::Value;
/// source: `export type InstanceHttpApiType` — shape as JSON value; CI verifies.
pub type InstanceHttpApiType = serde_json::Value;
