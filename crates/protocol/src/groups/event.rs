//! Rust port of `packages/protocol/src/groups/event.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `makeEventGroup(definitions)`, `EventGroup`
//! (built from `EventManifest.ServerDefinitions`), `OpenCodeEvent` +
//! `OpenCodeEventEncoded`, and `event.subscribe`
//! (`GET /api/event` SSE, identifier `v2.event.subscribe`).
//!
//! The event union schema itself is schema-owned (`definitions` are
//! `schema::event::Definition` values assembled schema-side). Source
//! behavior preserved: when `definitions` already contain a
//! `"server.connected"` definition it is used as-is, otherwise an empty
//! `"server.connected"` (`{}` data) member is appended and the union is
//! identified as `"V2Event"` (`"V2Event.server.connected"` for the appended
//! member). The group descriptor shape is identical either way, so
//! `make_event_group` takes no runtime arguments here; the definitions
//! selection lives schema-side.

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Port of `OpenCodeEvent` (the `V2Event` union `Type`-side).
pub type OpenCodeEvent = serde_json::Value;

/// Port of `OpenCodeEventEncoded` (the `V2Event` union `Encoded`-side).
pub type OpenCodeEventEncoded = serde_json::Value;

/// The appended member type verbatim from source (`"server.connected"`).
pub const SERVER_CONNECTED_TYPE: &str = "server.connected";

/// The union identifier verbatim from source (`"V2Event"`).
pub const OPEN_CODE_EVENT_IDENTIFIER: &str = "V2Event";

/// The appended member identifier verbatim from source
/// (`"V2Event.server.connected"`).
pub const SERVER_CONNECTED_IDENTIFIER: &str = "V2Event.server.connected";

/// Endpoint descriptors for `server.event`, in source order.
pub const EVENT_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "event.subscribe",
    openapi_identifier: "v2.event.subscribe",
    path: "/api/event",
    method: HttpMethod::GET,
    summary: Some("Subscribe to events"),
    description: Some("Subscribe to native event payloads for the server."),
    errors: &[],
}];

/// Group name verbatim from source (`HttpApiGroup.make("server.event")`).
pub const EVENT_GROUP_NAME: &str = "server.event";

/// Port of `makeEventGroup(definitions)`: returns the `server.event` group
/// descriptor. `definitions` are schema-side; see the module docs.
pub fn make_event_group() -> Group {
    Group {
        name: EVENT_GROUP_NAME,
        annotations: &[GroupAnnotation {
            title: Some("events"),
            description: Some("Experimental event stream route."),
        }],
        operations: EVENT_OPERATIONS,
    }
}

/// Port of `EventGroup` (`make(EventManifest.ServerDefinitions).group`).
#[allow(non_upper_case_globals)]
pub const EventGroup: Group = Group {
    name: EVENT_GROUP_NAME,
    annotations: &[GroupAnnotation {
        title: Some("events"),
        description: Some("Experimental event stream route."),
    }],
    operations: EVENT_OPERATIONS,
};
