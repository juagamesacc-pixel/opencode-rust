// source: src/server/routes/instance/httpapi/handlers/session.ts — exports: [sessionHandlers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/error`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/Stream` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/permission"
/// - "SessionHttpApi.list"
/// - "SessionHttpApi.status"
/// - "SessionHttpApi.requireSession"
/// - "SessionHttpApi.get"
/// - "SessionHttpApi.children"
/// - "SessionHttpApi.todo"
/// - "SessionHttpApi.diff"
/// source: `export const sessionHandlers` — shape as JSON value; CI verifies.
pub type sessionHandlers = serde_json::Value;
