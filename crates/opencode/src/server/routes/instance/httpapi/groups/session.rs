// source: src/server/routes/instance/httpapi/groups/session.ts — exports: [ListQuery, DiffQuery, MessagesQuery, StatusMap, UpdatePayload, ForkPayload, InitPayload, SummarizePayload, PromptPayload, CommandPayload, ShellPayload, RevertPayload, PermissionResponsePayload, SessionPaths, SessionApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
// PROVISIONAL pending crates/core: `@opencode-ai/core/model`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/permission"
/// - "/session"
/// - "sessionID"
/// - "List of sessions"
/// - "session.list"
/// - "List sessions"
/// - "Get a list of all OpenCode sessions, sorted by most recently updated."
/// - "Get session status"
/// source: `export const ListQuery` — shape as JSON value; CI verifies.
pub type ListQuery = serde_json::Value;
/// source: `export const DiffQuery` — shape as JSON value; CI verifies.
pub type DiffQuery = serde_json::Value;
/// source: `export const MessagesQuery` — shape as JSON value; CI verifies.
pub type MessagesQuery = serde_json::Value;
/// source: `export const StatusMap` — shape as JSON value; CI verifies.
pub type StatusMap = serde_json::Value;
/// source: `export const UpdatePayload` — shape as JSON value; CI verifies.
pub type UpdatePayload = serde_json::Value;
/// source: `export const ForkPayload` — shape as JSON value; CI verifies.
pub type ForkPayload = serde_json::Value;
/// source: `export const InitPayload` — shape as JSON value; CI verifies.
pub type InitPayload = serde_json::Value;
/// source: `export const SummarizePayload` — shape as JSON value; CI verifies.
pub type SummarizePayload = serde_json::Value;
/// source: `export const PromptPayload` — shape as JSON value; CI verifies.
pub type PromptPayload = serde_json::Value;
/// source: `export const CommandPayload` — shape as JSON value; CI verifies.
pub type CommandPayload = serde_json::Value;
/// source: `export const ShellPayload` — shape as JSON value; CI verifies.
pub type ShellPayload = serde_json::Value;
/// source: `export const RevertPayload` — shape as JSON value; CI verifies.
pub type RevertPayload = serde_json::Value;
/// source: `export const PermissionResponsePayload` — shape as JSON value; CI verifies.
pub type PermissionResponsePayload = serde_json::Value;
/// source: `export const SessionPaths` — shape as JSON value; CI verifies.
pub type SessionPaths = serde_json::Value;
/// source: `export const SessionApi` — shape as JSON value; CI verifies.
pub type SessionApi = serde_json::Value;
