// source: src/tool/truncate.ts — exports: [MAX_LINES, MAX_BYTES, DIR, GLOB, Result, Options, Interface, Service, node]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - ", agent.permission).action !== "
/// - "@opencode/Truncate"
/// - "Truncate.cleanup"
/// - "tool_"
/// - "Truncate.write"
/// - "Truncate.limits"
/// - "Truncate.output"
use serde::{Deserialize, Serialize};

/// source: `MAX_LINES = 2000` — verbatim.
pub const MAX_LINES: i64 = 2000;
/// source: `MAX_BYTES = 50` — verbatim.
pub const MAX_BYTES: i64 = 50;
/// source: `export const DIR` — shape as JSON value; CI verifies.
pub type DIR = serde_json::Value;
/// source: `export const GLOB` — shape as JSON value; CI verifies.
pub type GLOB = serde_json::Value;
/// source: `export type Result` — shape as JSON value; CI verifies.
pub type Result = serde_json::Value;
/// source: `export interface Options` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
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
