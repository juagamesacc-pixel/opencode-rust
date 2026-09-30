// source: src/server/routes/instance/httpapi/handlers/event.ts — exports: [eventHandlers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/event`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/Stream` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/encoding/Sse` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/event-v2-bridge"
/// - "server.instance.disposed"
/// - "10 seconds"
/// - "server.heartbeat"
/// - "event connected"
/// - "server.connected"
/// - "event disconnected"
/// - "text/event-stream"
/// source: `export const eventHandlers` — shape as JSON value; CI verifies.
pub type eventHandlers = serde_json::Value;
