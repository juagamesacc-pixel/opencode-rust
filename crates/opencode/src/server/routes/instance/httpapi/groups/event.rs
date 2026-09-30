// source: src/server/routes/instance/httpapi/groups/event.ts — exports: [EventPaths, EventApi]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "/event"
/// - "subscribe"
/// - "text/event-stream"
/// - "event.subscribe"
/// - "Subscribe to events"
/// - "Get events"
/// - "Instance event stream route."
/// source: `export const EventPaths` — shape as JSON value; CI verifies.
pub type EventPaths = serde_json::Value;
/// source: `export const EventApi` — shape as JSON value; CI verifies.
pub type EventApi = serde_json::Value;
