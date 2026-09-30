// source: src/cli/cmd/run/trace.ts — exports: [Trace, trace]
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
/// verbatim strings (source order, quoted for V2 audit):
/// - "latest.json"
/// - "trace.start"
/// source: `export type Trace` — shape as JSON value; CI verifies.
pub type Trace = serde_json::Value;
/// source: `export function trace` — stub shell; CI verifies behavior.
pub fn trace(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
