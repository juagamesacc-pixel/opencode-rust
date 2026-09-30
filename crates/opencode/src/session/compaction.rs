// source: src/session/compaction.ts — exports: [Event, PRUNE_MINIMUM, PRUNE_PROTECT, Interface, Service, use, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/config`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/service-use`
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
// PROVISIONAL pending crates/core: `@opencode-ai/core/model`
// PROVISIONAL pending crates/core: `@opencode-ai/core/session/compaction`
// PROVISIONAL pending crates/schema: `@opencode-ai/schema/session-compaction-event`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "reasoning"
/// - "completed"
/// - "[Old tool result content cleared]"
/// - "compaction"
/// - "assistant"
/// - "continue"
/// - "@opencode/SessionCompaction"
use serde::{Deserialize, Serialize};

/// source: `PRUNE_MINIMUM = 20_000` — verbatim.
pub const PRUNE_MINIMUM: i64 = 20000;
/// source: `PRUNE_PROTECT = 40_000` — verbatim.
pub const PRUNE_PROTECT: i64 = 40000;
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
/// source: `export const use` — shape as JSON value; CI verifies.
pub type r#use = serde_json::Value;
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
