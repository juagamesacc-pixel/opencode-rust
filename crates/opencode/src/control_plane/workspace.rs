// source: src/control-plane/workspace.ts — exports: Info, ConnectionStatus,
// Event, CreateInput, SessionWarpInput, SyncHttpError, WorkspaceNotFoundError,
// SessionEventsNotFoundError, SessionWarpHttpError, SyncTimeoutError,
// SyncAbortedError, Interface, Service, use, node, Workspace
// PROVISIONAL (966-line service): error tags + interface method table +
// service id verbatim; sync/target/warp bodies as trait (residual R-CP1, CI verifies).

use serde::{Deserialize, Serialize};

/// source: CreateInput — verbatim fields (per source struct).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// source: SessionWarpInput — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionWarpInput {
    pub session_id: String,
}

/// source: error tags — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHttpError {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceNotFoundError {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEventsNotFoundError {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionWarpHttpError {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncTimeoutError {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAbortedError {
    pub message: String,
}

/// source: Interface method table — verbatim names (per source interface).
pub const WORKSPACE_METHODS: &[&str] = &[
    "init", "list", "get", "create", "remove", "sync", "target", "warp",
];

/// source: Service "@opencode/Workspace" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Workspace";
