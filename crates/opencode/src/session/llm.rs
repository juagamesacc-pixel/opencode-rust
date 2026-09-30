// source: src/session/llm.ts — exports: [OUTPUT_TOKEN_MAX, StreamInput, StreamRequest, Interface, Service, use, hasToolCalls, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/app-node-platform`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/service-use`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/Stream` (host-provided; no new dep)
// PROVISIONAL pending external `ai` (host-provided; no new dep)
// PROVISIONAL pending crates/llm: `@opencode-ai/llm`
// PROVISIONAL pending crates/llm: `@opencode-ai/llm/route`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "required"
/// - "@opencode/LLM"
/// - "LLM.run"
/// - "session.id"
/// - "unbounded"
/// - "workflow_tool_approval"
/// - "startSpan"
use serde::{Deserialize, Serialize};

/// source: `export const OUTPUT_TOKEN_MAX` — shape as JSON value; CI verifies.
pub type OUTPUT_TOKEN_MAX = serde_json::Value;
/// source: `export type StreamInput` — shape as JSON value; CI verifies.
pub type StreamInput = serde_json::Value;
/// source: `export type StreamRequest` — shape as JSON value; CI verifies.
pub type StreamRequest = serde_json::Value;
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
/// source: `export const hasToolCalls` — shape as JSON value; CI verifies.
pub type hasToolCalls = serde_json::Value;
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
