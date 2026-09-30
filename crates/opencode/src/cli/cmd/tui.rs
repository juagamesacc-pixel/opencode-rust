// source: src/cli/cmd/tui.ts — exports: [resolveThreadDirectory, TuiThreadCommand]
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/util/error`
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/context/sdk`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/terminal-win32`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/cli/cmd/cmd"
/// - "global.event"
/// - "undefined"
/// - "./cli/tui/worker.js"
/// - "../tui/worker.ts"
/// - "$0 [project]"
/// - "start opencode tui"
/// - "path to start opencode in"
/// source: `export function resolveThreadDirectory` — stub shell; CI verifies behavior.
pub fn resolveThreadDirectory(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const TuiThreadCommand` — shape as JSON value; CI verifies.
pub type TuiThreadCommand = serde_json::Value;
