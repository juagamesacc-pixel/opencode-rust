// source: src/server/routes/instance/httpapi/websocket-tracker.ts — exports: [SERVER_CLOSING_EVENT, Interface, Service, node, register]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending external `effect/unstable/socket/Socket` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "server closing"
/// - "@opencode/HttpApiWebSocketTracker"
/// - "1 second"
/// - "unbounded"
use serde::{Deserialize, Serialize};

/// source: `export const SERVER_CLOSING_EVENT` — shape as JSON value; CI verifies.
pub type SERVER_CLOSING_EVENT = serde_json::Value;
/// source: `export interface Interface` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub value: serde_json::Value,
}
/// source: `export class Service` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub value: serde_json::Value,
}
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
/// source: `export const register` — shape as JSON value; CI verifies.
pub type register = serde_json::Value;
