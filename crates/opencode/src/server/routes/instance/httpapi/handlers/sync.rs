// source: src/server/routes/instance/httpapi/handlers/sync.ts — exports: [syncHandlers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/event`
// PROVISIONAL pending crates/core: `@opencode-ai/core/event/sql`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/control-plane/workspace"
/// - "SyncHttpApi.start"
/// - "SyncHttpApi.replay"
/// - "sync replay requested"
/// - "sync replay complete"
/// - "SyncHttpApi.steal"
/// - "sync session stolen"
/// - "SyncHttpApi.history"
/// source: `export const syncHandlers` — shape as JSON value; CI verifies.
pub type syncHandlers = serde_json::Value;
