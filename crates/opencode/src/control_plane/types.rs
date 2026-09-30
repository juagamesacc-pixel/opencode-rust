// source: src/control-plane/types.ts — exports: WorkspaceInfo,
// WorkspaceListedInfo, WorkspaceAdapterEntry, Target, WorkspaceAdapterContext,
// WorkspaceAdapter ("Workspace" identifier; listed omits id; verbatim).

use serde::{Deserialize, Serialize};

/// source: WorkspaceInfo ("Workspace") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Option<serde_json::Value>>,
    pub project_id: String,
}

/// source: WorkspaceAdapterEntry { type, name, description } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceAdapterEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub description: String,
}

/// source: Target local|remote — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Target {
    #[serde(rename = "local")]
    Local { directory: String },
    #[serde(rename = "remote")]
    Remote { url: String },
}

/// source: WorkspaceAdapter method table — verbatim names/order.
pub const ADAPTER_METHODS: &[&str] = &["configure", "create", "list", "remove", "target"];
