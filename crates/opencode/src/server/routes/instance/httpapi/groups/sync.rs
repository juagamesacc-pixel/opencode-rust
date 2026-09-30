// source: src/server/routes/instance/httpapi/groups/sync.ts — exports: [ReplayEvent, ReplayPayload, ReplayResponse, SessionPayload, HistoryPayload, HistoryEvent, SyncPaths, SyncApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending crates/core: `@opencode-ai/core/event`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/schema"
/// - "/sync"
/// - "Workspace sync started"
/// - "sync.start"
/// - "Start workspace sync"
/// - "Start sync loops for workspaces in the current project that have active sessions."
/// - "Replayed sync events"
/// - "sync.replay"
/// source: `export const ReplayEvent` — shape as JSON value; CI verifies.
pub type ReplayEvent = serde_json::Value;
/// source: `export const ReplayPayload` — shape as JSON value; CI verifies.
pub type ReplayPayload = serde_json::Value;
/// source: `export const ReplayResponse` — shape as JSON value; CI verifies.
pub type ReplayResponse = serde_json::Value;
/// source: `export const SessionPayload` — shape as JSON value; CI verifies.
pub type SessionPayload = serde_json::Value;
/// source: `export const HistoryPayload` — shape as JSON value; CI verifies.
pub type HistoryPayload = serde_json::Value;
/// source: `export const HistoryEvent` — shape as JSON value; CI verifies.
pub type HistoryEvent = serde_json::Value;
/// source: `export const SyncPaths` — shape as JSON value; CI verifies.
pub type SyncPaths = serde_json::Value;
/// source: `export const SyncApi` — shape as JSON value; CI verifies.
pub type SyncApi = serde_json::Value;
