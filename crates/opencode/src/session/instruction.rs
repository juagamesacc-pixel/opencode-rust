// source: src/session/instruction.ts — exports: [Interface, Service, loaded, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/app-node-platform`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/flag/flag`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "completed"
/// - "@opencode/Instruction"
/// - "AGENTS.md"
/// - ".claude"
/// - "CLAUDE.md"
/// - "CONTEXT.md"
/// - "Instruction.state"
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
/// source: `export function loaded` — stub shell; CI verifies behavior.
pub fn loaded(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
