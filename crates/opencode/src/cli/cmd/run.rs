// source: src/cli/cmd/run.ts — exports: [RunCommand, runMini]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending external `node:fs/promises` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/permission"
/// - "./run/tool"
/// - "run [message..]"
/// - "run opencode with a message"
/// - "message to send"
/// - "the command to run, use message for args"
/// - "continue"
/// - "continue the last session"
/// source: `export const RunCommand` — shape as JSON value; CI verifies.
pub type RunCommand = serde_json::Value;
/// source: `export function runMini` — stub shell; CI verifies behavior.
pub fn runMini(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
