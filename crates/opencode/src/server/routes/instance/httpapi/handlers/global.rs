// source: src/server/routes/instance/httpapi/handlers/global.ts — exports: [globalHandlers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/event`
// PROVISIONAL pending crates/core: `@opencode-ai/core/installation/version`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/Stream` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/encoding/Sse` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/config/config"
/// - "global event connected"
/// - "10 seconds"
/// - "server.heartbeat"
/// - "server.connected"
/// - "global event disconnected"
/// - "text/event-stream"
/// - "Cache-Control"
/// source: `export const globalHandlers` — shape as JSON value; CI verifies.
pub type globalHandlers = serde_json::Value;
