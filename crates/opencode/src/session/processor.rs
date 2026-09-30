// source: src/session/processor.ts — exports: [Result, Handle, Interface, Service, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/Stream` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/llm: `@opencode-ai/llm`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "continue"
/// - "messageID"
/// - "sessionID"
/// - "@opencode/SessionProcessor"
/// - "SessionProcessor.create"
/// - "SessionProcessor.settleToolCall"
/// - "SessionProcessor.readToolCall"
use serde::{Deserialize, Serialize};

/// source: `export type Result` — shape as JSON value; CI verifies.
pub type Result = serde_json::Value;
/// source: `export interface Handle` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handle {
    pub value: serde_json::Value,
}
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
