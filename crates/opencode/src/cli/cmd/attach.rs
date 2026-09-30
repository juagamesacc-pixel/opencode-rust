// source: src/cli/cmd/attach.ts — exports: [AttachCommand]
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/util/error`
/// verbatim strings (source order, quoted for V2 audit):
/// - "./cmd"
/// - "attach <url>"
/// - "attach to a running opencode server"
/// - "http://localhost:4096"
/// - "directory to run in"
/// - "continue"
/// - "continue the last session"
/// - "session id to continue"
/// source: `export const AttachCommand` — shape as JSON value; CI verifies.
pub type AttachCommand = serde_json::Value;
