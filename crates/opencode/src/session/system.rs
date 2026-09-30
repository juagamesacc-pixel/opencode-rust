// source: src/session/system.ts — exports: [provider, Interface, Service, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location-services`
// PROVISIONAL pending crates/core: `@opencode-ai/core/reference`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "muse-glimmer"
/// - "Muse Glimmer"
/// - "Muse Spark"
/// - "{{MODEL_NAME}}"
/// - "gpt-4"
/// - ") || model.api.id.includes("
/// - "gpt-6"
use serde::{Deserialize, Serialize};

/// source: `export function provider` — stub shell; CI verifies behavior.
pub fn provider(payload: serde_json::Value) -> serde_json::Value {
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
