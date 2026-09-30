// source: src/cli/cmd/run/runtime.stdin.ts — exports: [INTERACTIVE_INPUT_ERROR, resolveInteractiveStdin]
// PROVISIONAL pending external `node:tty` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "--mini requires a controlling terminal for input"
/// - "/dev/tty"
/// source: `INTERACTIVE_INPUT_ERROR = "--mini requires a controlling terminal for input"` — verbatim.
pub const INTERACTIVE_INPUT_ERROR: &str = "--mini requires a controlling terminal for input";
/// source: `export function resolveInteractiveStdin` — stub shell; CI verifies behavior.
pub fn resolveInteractiveStdin(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
