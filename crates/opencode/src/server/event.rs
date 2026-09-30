// source: src/server/event.ts — exports: [Event, InstanceDisposed]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/schema: `@opencode-ai/schema/server-event`
/// verbatim strings (source order, quoted for V2 audit):
/// - "server.instance.disposed"
/// - "Event.server.instance.disposed"
/// source: `export const Event` — shape as JSON value; CI verifies.
pub type Event = serde_json::Value;
/// source: `export const InstanceDisposed` — shape as JSON value; CI verifies.
pub type InstanceDisposed = serde_json::Value;
