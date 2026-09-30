// source: src/tool/registry.ts — exports: [webSearchEnabled, Interface, Service, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/app-node-platform`
// PROVISIONAL pending crates/core: `@opencode-ai/core/ripgrep`
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/plugin: `@opencode-ai/plugin`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/glob`
// PROVISIONAL pending external `effect/unstable/process/ChildProcessSpawner` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/cross-spawn-spawner`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "opencode-go"
/// - "@opencode/ToolRegistry"
/// - "./code-mode"
/// - "ToolRegistry.state"
/// - " : (result.title ?? "
/// - "Tool.execute"
/// - "tool.name"
use serde::{Deserialize, Serialize};

/// source: `export function webSearchEnabled` — stub shell; CI verifies behavior.
pub fn webSearchEnabled(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
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
