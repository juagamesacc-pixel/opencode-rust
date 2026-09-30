// source: src/server/routes/instance/httpapi/groups/pty.ts — exports: [Params, CursorQuery, ShellItem, PtyPaths, PtyApi, PtyConnectApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/pty`
// PROVISIONAL pending crates/core: `@opencode-ai/core/pty/ticket`
// PROVISIONAL pending crates/core: `@opencode-ai/core/pty/schema`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/pty"
/// - "/pty"
/// - "List of shells"
/// - "pty.shells"
/// - "List available shells"
/// - "Get a list of available shells on the system."
/// - "List of sessions"
/// - "pty.list"
/// source: `export const Params` — shape as JSON value; CI verifies.
pub type Params = serde_json::Value;
/// source: `export const CursorQuery` — shape as JSON value; CI verifies.
pub type CursorQuery = serde_json::Value;
/// source: `export const ShellItem` — shape as JSON value; CI verifies.
pub type ShellItem = serde_json::Value;
/// source: `export const PtyPaths` — shape as JSON value; CI verifies.
pub type PtyPaths = serde_json::Value;
/// source: `export const PtyApi` — shape as JSON value; CI verifies.
pub type PtyApi = serde_json::Value;
/// source: `export const PtyConnectApi` — shape as JSON value; CI verifies.
pub type PtyConnectApi = serde_json::Value;
