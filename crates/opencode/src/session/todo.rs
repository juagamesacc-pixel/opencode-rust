// source: src/session/todo.ts — exports: [Info, Event, Interface, Service, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/session/sql`
// PROVISIONAL pending crates/schema: `@opencode-ai/schema/session-todo`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "@opencode/SessionTodo"
/// - "Todo.update"
/// - "Todo.get"
use serde::{Deserialize, Serialize};

/// source: `export const Info` — shape as JSON value; CI verifies.
pub type Info = serde_json::Value;
/// source: `export const Event` — shape as JSON value; CI verifies.
pub type Event = serde_json::Value;
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
