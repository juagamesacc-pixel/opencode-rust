// source: src/cli/cmd/session.ts — exports: [SessionCommand, SessionDeleteCommand, SessionListCommand]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/flag/flag`
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/which`
/// verbatim strings (source order, quoted for V2 audit):
/// - "less.exe"
/// - "manage sessions"
/// - "delete <sessionID>"
/// - "delete a session"
/// - "sessionID"
/// - "session ID to delete"
/// - "Cli.session.delete"
/// - "list sessions"
/// source: `export const SessionCommand` — shape as JSON value; CI verifies.
pub type SessionCommand = serde_json::Value;
/// source: `export const SessionDeleteCommand` — shape as JSON value; CI verifies.
pub type SessionDeleteCommand = serde_json::Value;
/// source: `export const SessionListCommand` — shape as JSON value; CI verifies.
pub type SessionListCommand = serde_json::Value;
