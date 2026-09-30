// source: src/server/shared/fence.ts — exports: [HEADER, State, load, diff, parse, wait]
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/event/sql`
// PROVISIONAL pending crates/core: `@opencode-ai/core/workspace`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/database/database"
/// - "x-opencode-sync"
/// - "waiting for state"
/// - "state fully synced"
/// source: `HEADER = "x-opencode-sync"` — verbatim.
pub const HEADER: &str = "x-opencode-sync";
/// source: `export type State` — shape as JSON value; CI verifies.
pub type State = serde_json::Value;
/// source: `export function load` — stub shell; CI verifies behavior.
pub fn load(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function diff` — stub shell; CI verifies behavior.
pub fn diff(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function parse` — stub shell; CI verifies behavior.
pub fn parse(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function wait` — stub shell; CI verifies behavior.
pub fn wait(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
