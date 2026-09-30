// source: src/server/routes/instance/httpapi/groups/experimental.ts — exports: [ConsoleSwitchPayload, ToolListQuery, WorktreeApiError, SessionListQuery, ExperimentalPaths, ExperimentalApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
// PROVISIONAL pending crates/core: `@opencode-ai/core/model`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/account/schema"
/// - "ConsoleState"
/// - "ExperimentalCapabilities"
/// - "ToolListItem"
/// - "ToolList"
/// - "WorktreeNotGitError"
/// - "WorktreeNameGenerationFailedError"
/// - "WorktreeCreateFailedError"
use serde::{Deserialize, Serialize};

/// source: `export const ConsoleSwitchPayload` — shape as JSON value; CI verifies.
pub type ConsoleSwitchPayload = serde_json::Value;
/// source: `export const ToolListQuery` — shape as JSON value; CI verifies.
pub type ToolListQuery = serde_json::Value;
/// source: `export class WorktreeApiError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeApiError {
    pub value: serde_json::Value,
}
/// source: `export const SessionListQuery` — shape as JSON value; CI verifies.
pub type SessionListQuery = serde_json::Value;
/// source: `export const ExperimentalPaths` — shape as JSON value; CI verifies.
pub type ExperimentalPaths = serde_json::Value;
/// source: `export const ExperimentalApi` — shape as JSON value; CI verifies.
pub type ExperimentalApi = serde_json::Value;
