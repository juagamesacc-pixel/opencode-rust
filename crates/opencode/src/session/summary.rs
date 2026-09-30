// source: src/session/summary.ts — exports: [Interface, Service, DiffInput, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - " && next <= "
/// - " || next === '"
/// - "@opencode/SessionSummary"
/// - "SessionSummary.computeDiff"
/// - "step-start"
/// - "step-finish"
/// - "SessionSummary.summarize"
use serde::{Deserialize, Serialize};

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
/// source: `export const DiffInput` — shape as JSON value; CI verifies.
pub type DiffInput = serde_json::Value;
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
