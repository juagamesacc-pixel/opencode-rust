//! Rust port of `packages/server/src/handlers/event.ts` (opencode v1.18.30).
//!
//! Source 40 lines: `EventHandler` with `event.subscribe` raw handler producing
//! SSE stream (subscriberCapacity=256, connected event, Sse.encode, heartbeat 15s, headers).
//!
//! PROVISIONAL: `EventV2.Service` pending `crates/core`.

/// Capacity verbatim: 256.
pub const SUBSCRIBER_CAPACITY: usize = 256;
pub const GROUP: &str = "server.event";
pub const OPERATION: &str = "event.subscribe";
pub const RAW_HANDLER: bool = true;
/// Heartbeat interval verbatim: "15 seconds".
pub const HEARTBEAT_INTERVAL: &str = "15 seconds";
/// SSE headers verbatim.
pub const SSE_HEADERS: &[(&str, &str)] = &[
    ("Cache-Control", "no-cache, no-transform"),
    ("X-Accel-Buffering", "no"),
    ("X-Content-Type-Options", "nosniff"),
];
pub const CONTENT_TYPE: &str = "text/event-stream";
pub const CONNECTED_EVENT_TYPE: &str = "server.connected";
