// source: src/server/routes/instance/httpapi/middleware/workspace-routing.ts — exports: [WorkspaceRoutingQueryFields, WorkspaceRoutingQuery, WorkspaceRouteContext, WorkspaceRoutingMiddleware, workspaceRoutingLayer]
// PROVISIONAL pending crates/core: `@opencode-ai/core/workspace`
// PROVISIONAL pending crates/core: `@opencode-ai/core/flag/flag`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/socket/Socket` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/workspace"
/// - "InvalidWorkspaceID"
/// - "@opencode/ExperimentalHttpApiWorkspaceRouteContext"
/// - "@opencode/ExperimentalHttpApiWorkspaceRouting"
/// - "http://localhost"
/// - "workspace"
/// - "directory"
/// - "x-opencode-directory"
use serde::{Deserialize, Serialize};

/// source: `export const WorkspaceRoutingQueryFields` — shape as JSON value; CI verifies.
pub type WorkspaceRoutingQueryFields = serde_json::Value;
/// source: `export const WorkspaceRoutingQuery` — shape as JSON value; CI verifies.
pub type WorkspaceRoutingQuery = serde_json::Value;
/// source: `export class WorkspaceRouteContext` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRouteContext {
    pub value: serde_json::Value,
}
/// source: `export class WorkspaceRoutingMiddleware` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRoutingMiddleware {
    pub value: serde_json::Value,
}
/// source: `export const workspaceRoutingLayer` — shape as JSON value; CI verifies.
pub type workspaceRoutingLayer = serde_json::Value;
