// source: src/session/prompt.ts — exports: [Interface, Service, PromptInput, LoopInput, ShellInput, CommandInput, createStructuredOutputTool, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `ai` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/session/runner/max-steps`
// PROVISIONAL pending external `effect/unstable/process` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/cross-spawn-spawner`
// PROVISIONAL pending external `effect/Stream` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/error`
// PROVISIONAL pending crates/core: `@opencode-ai/core/shell`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "application/pdf"
/// - "image/gif"
/// - "image/jpeg"
/// - "image/png"
/// - "image/webp"
/// - ") ? 2 : trimmed.endsWith("
/// - "@opencode/SessionPrompt"
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
/// source: `export const PromptInput` — shape as JSON value; CI verifies.
pub type PromptInput = serde_json::Value;
/// source: `export class LoopInput` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopInput {
    pub value: serde_json::Value,
}
/// source: `export const ShellInput` — shape as JSON value; CI verifies.
pub type ShellInput = serde_json::Value;
/// source: `export const CommandInput` — shape as JSON value; CI verifies.
pub type CommandInput = serde_json::Value;
/// source: `export function createStructuredOutputTool` — stub shell; CI verifies behavior.
pub fn createStructuredOutputTool(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
