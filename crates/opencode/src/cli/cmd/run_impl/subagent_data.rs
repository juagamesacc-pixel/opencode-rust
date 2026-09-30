// source: src/cli/cmd/run/subagent-data.ts — exports: [SUBAGENT_BOOTSTRAP_LIMIT, SUBAGENT_CALL_BOOTSTRAP_LIMIT, SubagentData, BootstrapSubagentInput, sameSubagentTab, listSubagentPermissions, listSubagentQuestions, createSubagentData, listSubagentTabs, snapshotSubagentData, snapshotSelectedSubagentData, bootstrapSubagentData, bootstrapSubagentCalls, reduceSubagentData]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/sdk/v2"
/// - "completed"
/// - "metadata"
/// - "interrupted"
/// - "Tool execution aborted"
/// - "cancelled"
/// - "background"
/// - "toolcalls"
/// source: `SUBAGENT_BOOTSTRAP_LIMIT = 200` — verbatim.
pub const SUBAGENT_BOOTSTRAP_LIMIT: i64 = 200;
/// source: `SUBAGENT_CALL_BOOTSTRAP_LIMIT = 80` — verbatim.
pub const SUBAGENT_CALL_BOOTSTRAP_LIMIT: i64 = 80;
/// source: `export type SubagentData` — shape as JSON value; CI verifies.
pub type SubagentData = serde_json::Value;
/// source: `export type BootstrapSubagentInput` — shape as JSON value; CI verifies.
pub type BootstrapSubagentInput = serde_json::Value;
/// source: `export function sameSubagentTab` — stub shell; CI verifies behavior.
pub fn sameSubagentTab(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function listSubagentPermissions` — stub shell; CI verifies behavior.
pub fn listSubagentPermissions(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function listSubagentQuestions` — stub shell; CI verifies behavior.
pub fn listSubagentQuestions(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function createSubagentData` — stub shell; CI verifies behavior.
pub fn createSubagentData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function listSubagentTabs` — stub shell; CI verifies behavior.
pub fn listSubagentTabs(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function snapshotSubagentData` — stub shell; CI verifies behavior.
pub fn snapshotSubagentData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function snapshotSelectedSubagentData` — stub shell; CI verifies behavior.
pub fn snapshotSelectedSubagentData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function bootstrapSubagentData` — stub shell; CI verifies behavior.
pub fn bootstrapSubagentData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function bootstrapSubagentCalls` — stub shell; CI verifies behavior.
pub fn bootstrapSubagentCalls(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function reduceSubagentData` — stub shell; CI verifies behavior.
pub fn reduceSubagentData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
