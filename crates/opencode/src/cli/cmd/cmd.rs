// source: src/cli/cmd/cmd.ts — exports: [WithDoubleDash, cmd]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
/// source: `export type WithDoubleDash` — shape as JSON value; CI verifies.
pub type WithDoubleDash = serde_json::Value;
/// source: `export function cmd` — stub shell; CI verifies behavior.
pub fn cmd(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
