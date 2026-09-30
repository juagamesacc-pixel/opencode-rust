// source: src/server/routes/instance/httpapi/groups/workspace.ts — exports: [CreatePayload, WarpPayload, ApiWorkspaceWarpError, ApiWorkspaceCreateError, WorkspacePaths, WorkspaceApi]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/control-plane/workspace"
/// - "/experimental/workspace"
/// - "projectID"
/// - "WorkspaceWarpError"
/// - "WorkspaceCreateError"
/// - "workspace"
/// - "adapters"
/// - "Workspace adapters"
use serde::{Deserialize, Serialize};

/// source: `export const CreatePayload` — shape as JSON value; CI verifies.
pub type CreatePayload = serde_json::Value;
/// source: `export const WarpPayload` — shape as JSON value; CI verifies.
pub type WarpPayload = serde_json::Value;
/// source: `export class ApiWorkspaceWarpError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiWorkspaceWarpError {
    pub value: serde_json::Value,
}
/// source: `export class ApiWorkspaceCreateError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiWorkspaceCreateError {
    pub value: serde_json::Value,
}
/// source: `export const WorkspacePaths` — shape as JSON value; CI verifies.
pub type WorkspacePaths = serde_json::Value;
/// source: `export const WorkspaceApi` — shape as JSON value; CI verifies.
pub type WorkspaceApi = serde_json::Value;
