// source: src/session/revert.ts — exports: [RevertInput, Interface, Service, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "@opencode/SessionRevert"
/// - "SessionRevert.revert"
/// - "session_diff"
/// - "SessionRevert.unrevert"
/// - "unreverting"
/// - "SessionRevert.cleanup"
use serde::{Deserialize, Serialize};

/// source: `export const RevertInput` — shape as JSON value; CI verifies.
pub type RevertInput = serde_json::Value;
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
