// source: src/tool/tool.ts — exports: [DynamicDescription, InvalidArgumentsError, Context, ExecuteResult, Def, DefWithoutID, Info, InferParameters, InferMetadata, InferDef, define, init]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/permission"
/// - "rewrite the input"
/// - "ToolInvalidArgumentsError"
/// - "sessionID"
/// - "messageID"
/// - "function"
/// - "tool.name"
/// - "session.id"
use serde::{Deserialize, Serialize};

/// source: `export type DynamicDescription` — shape as JSON value; CI verifies.
pub type DynamicDescription = serde_json::Value;
/// source: `export class InvalidArgumentsError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidArgumentsError {
    pub value: serde_json::Value,
}
/// source: `export type Context` — shape as JSON value; CI verifies.
pub type Context = serde_json::Value;
/// source: `export interface ExecuteResult` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResult {
    pub value: serde_json::Value,
}
/// source: `export interface Def` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Def {
    pub value: serde_json::Value,
}
/// source: `export type DefWithoutID` — shape as JSON value; CI verifies.
pub type DefWithoutID = serde_json::Value;
/// source: `export interface Info` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub value: serde_json::Value,
}
/// source: `export type InferParameters` — shape as JSON value; CI verifies.
pub type InferParameters = serde_json::Value;
/// source: `export type InferMetadata` — shape as JSON value; CI verifies.
pub type InferMetadata = serde_json::Value;
/// source: `export type InferDef` — shape as JSON value; CI verifies.
pub type InferDef = serde_json::Value;
/// source: `export function define` — stub shell; CI verifies behavior.
pub fn define(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function init` — stub shell; CI verifies behavior.
pub fn init(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
