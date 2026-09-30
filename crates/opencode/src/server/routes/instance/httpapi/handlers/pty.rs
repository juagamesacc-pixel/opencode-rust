// source: src/server/routes/instance/httpapi/handlers/pty.ts — exports: [ptyHandlers, ptyConnectHandlers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/pty`
// PROVISIONAL pending crates/core: `@opencode-ai/core/pty/protocol`
// PROVISIONAL pending crates/core: `@opencode-ai/core/pty/schema`
// PROVISIONAL pending crates/core: `@opencode-ai/core/pty/ticket`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location-services`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location`
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending crates/core: `@opencode-ai/core/shell`
// PROVISIONAL pending crates/server: `@opencode-ai/server/cors`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/effect/instance-state"
/// - "PtyHttpApi.shells"
/// - "PtyHttpApi.list"
/// - "PtyHttpApi.create"
/// - "shell.env"
/// - "PtyHttpApi.get"
/// - "Pty.NotFoundError"
/// - "PtyHttpApi.update"
/// source: `export const ptyHandlers` — shape as JSON value; CI verifies.
pub type ptyHandlers = serde_json::Value;
/// source: `export const ptyConnectHandlers` — shape as JSON value; CI verifies.
pub type ptyConnectHandlers = serde_json::Value;
