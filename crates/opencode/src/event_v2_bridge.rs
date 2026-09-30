// source: src/event-v2-bridge.ts — exports: Service, node, EventV2Bridge
// (publish location-attach: options.location passthrough, else InstanceRef +
// WorkspaceRef → Location.Info; listen fan-out to GlobalBus "event" +
// durable "sync" envelope with versionedType/seq/aggregateID; verbatim).
// PROVISIONAL pending core (layer-node, event, location, project, schema) + @/*.

/// source: Service "@opencode/EventV2Bridge" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/EventV2Bridge";

/// source: node deps [EventV2.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@opencode-ai/core/event.EventV2"];

/// source: sync envelope type "sync" — verbatim.
pub const SYNC_TYPE: &str = "sync";
