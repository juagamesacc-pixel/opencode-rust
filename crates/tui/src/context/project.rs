// source: packages/tui/src/context/project.tsx (115 lines, v1.18.30)
// 1:1 port — the store is an explicit struct; `sync`/`syncWorkspace` keep
// their call order and batching; the `workspace.status` event is handled
// via `handle_event` (wired by the app loop).

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use super::event::SdkEventEnvelope;
use super::sdk::{data2, SdkClient};

/// Mirrors the SDK `Path` shape read here.
#[derive(Debug, Clone, Default)]
pub struct InstancePath {
    pub home: String,
    pub state: String,
    pub config: String,
    pub worktree: String,
    pub directory: String,
}

impl InstancePath {
    fn from_value(value: &Value, fallback_directory: &str) -> Self {
        let get = |key: &str| {
            value
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };
        Self {
            home: get("home"),
            state: get("state"),
            config: get("config"),
            worktree: get("worktree"),
            directory: value
                .get("directory")
                .and_then(|v| v.as_str())
                .unwrap_or(fallback_directory)
                .to_string(),
        }
    }
}

/// Mirrors `WorkspaceStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceStatus {
    Connected,
    Connecting,
    Disconnected,
    Error,
}

impl WorkspaceStatus {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "connected" => Some(WorkspaceStatus::Connected),
            "connecting" => Some(WorkspaceStatus::Connecting),
            "disconnected" => Some(WorkspaceStatus::Disconnected),
            "error" => Some(WorkspaceStatus::Error),
            _ => None,
        }
    }
}

/// Mirrors the Project context value.
pub struct ProjectStore {
    project_id: Option<String>,
    worktree: Option<String>,
    main_dir: Option<String>,
    instance_path: InstancePath,
    workspace_current: Option<String>,
    workspace_list: Vec<Value>,
    workspace_status: HashMap<String, WorkspaceStatus>,
    client: Arc<dyn SdkClient>,
}

impl ProjectStore {
    pub fn new(client: Arc<dyn SdkClient>, directory: Option<String>) -> Self {
        let dir = directory.unwrap_or_default();
        Self {
            project_id: None,
            worktree: None,
            main_dir: None,
            instance_path: InstancePath {
                directory: dir,
                ..InstancePath::default()
            },
            workspace_current: None,
            workspace_list: Vec::new(),
            workspace_status: HashMap::new(),
            client,
        }
    }

    fn workspace_param(&self) -> Value {
        match &self.workspace_current {
            Some(current) => serde_json::json!({ "workspace": current }),
            None => serde_json::json!({}),
        }
    }

    /// Mirrors `sync`.
    pub async fn sync(&mut self) -> Result<(), String> {
        let param = self.workspace_param();
        let (instance_path, project) = tokio::join!(
            self.client.call("path.get", param.clone()),
            self.client.call("project.current", param.clone()),
        );
        let project_data = data2(&project?).cloned().unwrap_or(Value::Null);
        let project_id = project_data
            .get("id")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let directories = match &project_id {
            Some(id) => {
                let response = self
                    .client
                    .call(
                        "project.directories",
                        serde_json::json!({ "projectID": id, "workspace": param.get("workspace") }),
                    )
                    .await?;
                data2(&response).cloned()
            }
            None => None,
        };
        // Batch the store writes (mirrors Solid `batch`).
        let fallback = self.instance_path.directory.clone();
        self.instance_path = InstancePath::from_value(
            instance_path?.get("data").unwrap_or(&Value::Null),
            &fallback,
        );
        self.project_id = project_id;
        self.worktree = project_data
            .get("worktree")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        self.main_dir = directories
            .as_ref()
            .and_then(|d| d.as_array())
            .and_then(|items| {
                items
                    .iter()
                    .rev()
                    .find(|item| item.get("strategy").is_none())
                    .and_then(|item| {
                        item.get("directory")
                            .and_then(|v| v.as_str())
                            .map(str::to_string)
                    })
            });
        Ok(())
    }

    /// Mirrors `syncWorkspace`.
    pub async fn sync_workspace(&mut self) -> Result<(), String> {
        let listed = match self
            .client
            .call("experimental.workspace.list", serde_json::json!({}))
            .await
        {
            Ok(response) => response,
            Err(_) => return Ok(()),
        };
        let data = data2(&listed)
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        if data.is_empty() && data2(&listed).is_none() {
            return Ok(());
        }
        let status = self
            .client
            .call("experimental.workspace.status", serde_json::json!({}))
            .await
            .ok();
        let next: HashMap<String, WorkspaceStatus> = status
            .as_ref()
            .and_then(data2)
            .and_then(|d| d.as_array())
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        Some((
                            item.get("workspaceID")?.as_str()?.to_string(),
                            WorkspaceStatus::parse(item.get("status")?.as_str()?)?,
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.workspace_list = data;
        self.workspace_status = next;
        if !self.workspace_list.iter().any(|item| {
            item.get("id")
                .and_then(|v| v.as_str())
                .map(|id| Some(id) == self.workspace_current.as_deref())
                .unwrap_or(false)
        }) {
            self.workspace_current = None;
        }
        Ok(())
    }

    /// Mirrors the `workspace.status` event subscription.
    pub fn handle_event(&mut self, event: &SdkEventEnvelope) {
        if event.payload.get("type").and_then(|t| t.as_str()) != Some("workspace.status") {
            return;
        }
        let properties = event.payload.get("properties").unwrap_or(&Value::Null);
        let (id, status) = (
            properties.get("workspaceID").and_then(|v| v.as_str()),
            properties.get("status").and_then(|v| v.as_str()),
        );
        if let (Some(id), Some(status)) = (id, status) {
            if let Some(parsed) = WorkspaceStatus::parse(status) {
                self.workspace_status.insert(id.to_string(), parsed);
            }
        }
    }

    pub fn project(&self) -> Option<&str> {
        self.project_id.as_deref()
    }

    pub fn instance_path(&self) -> &InstancePath {
        &self.instance_path
    }

    pub fn instance_directory(&self) -> &str {
        &self.instance_path.directory
    }

    pub fn workspace_current(&self) -> Option<&str> {
        self.workspace_current.as_deref()
    }

    pub fn workspace_set(&mut self, next: Option<String>) {
        if self.workspace_current == next {
            return;
        }
        self.workspace_current = next;
    }

    pub fn workspace_list(&self) -> &[Value] {
        &self.workspace_list
    }

    pub fn workspace_get(&self, workspace_id: &str) -> Option<&Value> {
        self.workspace_list
            .iter()
            .find(|item| item.get("id").and_then(|v| v.as_str()) == Some(workspace_id))
    }

    pub fn workspace_status(&self, workspace_id: &str) -> Option<WorkspaceStatus> {
        self.workspace_status.get(workspace_id).copied()
    }

    pub fn workspace_statuses(&self) -> &HashMap<String, WorkspaceStatus> {
        &self.workspace_status
    }
}
