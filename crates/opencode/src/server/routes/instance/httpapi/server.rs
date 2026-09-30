// source: src/server/routes/instance/httpapi/server.ts — exports: [context, createRoutes, routes, webHandler]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/socket/Socket` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending crates/core: `@opencode-ai/core/observability`
// PROVISIONAL pending crates/core: `@opencode-ai/core/control-plane/move-session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/app-node-platform`
/// verbatim strings (source order, quoted for V2 audit):
/// - "/doc"
/// - "GlobalError"
/// - "Requires"
/// - "GlobalRequires"
/// source: `export const context` — shape as JSON value; CI verifies.
pub type context = serde_json::Value;
/// source: `export function createRoutes` — stub shell; CI verifies behavior.
pub fn createRoutes(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const routes` — shape as JSON value; CI verifies.
pub type routes = serde_json::Value;
/// source: `export const webHandler` — shape as JSON value; CI verifies.
pub type webHandler = serde_json::Value;
