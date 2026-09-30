// source: packages/tui/src/component/dialog-workspace-create.tsx (308 lines, v1.18.30)
// 1:1 port — `recentConnectedWorkspaces` (connected filter, timeUsed
// desc, limit 3, omit), `warpReminderText`, adapter loading with the
// verbatim failure toast, `openWorkspaceSelect` / `warpWorkspaceSession` /
// `confirmWorkspaceFileChanges` flows, and both select builders.

#![allow(dead_code)]

use serde_json::Value;
use std::sync::Arc;

use super::dialog_workspace_file_changes::{show_file_changes, FileChangesChoice};
use crate::context::project::ProjectStore;
use crate::context::sdk::SdkClient;
use crate::context::sync::SyncStore;
use crate::ui::dialog::DialogStack;
use crate::ui::dialog_alert::show_alert;
use crate::ui::dialog_select::{SelectOption, SelectState};
use crate::ui::toast::{ToastInput, ToastState};
use crate::util::error::error_message_value;
use crate::util::selection::ToastVariant;

/// Mirrors `WorkspaceSelection`.
#[derive(Debug, Clone)]
pub enum WorkspaceSelection {
    None,
    New {
        workspace_type: String,
        workspace_name: String,
    },
    Existing {
        workspace_id: String,
        workspace_type: String,
        workspace_name: String,
    },
}

/// Mirrors the `{ type: "existing-list" }` escape hatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceSelectValue {
    None,
    New,
    ExistingList,
}

/// Mirrors `recentConnectedWorkspaces`.
pub fn recent_connected_workspaces(
    workspaces: &[Value],
    status: impl Fn(&str) -> Option<String>,
    limit: usize,
    omit_workspace_id: Option<&str>,
) -> (Vec<Value>, bool) {
    let mut connected: Vec<Value> = workspaces
        .iter()
        .filter(|w| {
            let id = w.get("id").and_then(|v| v.as_str()).unwrap_or("");
            status(id).as_deref() == Some("connected") && Some(id) != omit_workspace_id
        })
        .cloned()
        .collect();
    connected.sort_by(|a, b| {
        let time = |w: &Value| {
            w.get("timeUsed")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0)
        };
        time(b)
            .partial_cmp(&time(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let has_more = connected.len() > limit;
    (connected.into_iter().take(limit).collect(), has_more)
}

/// Mirrors `warpReminderText`.
pub fn warp_reminder_text(dir: &str) -> String {
    format!("<system-reminder>The user has changed the current working directory to \"{dir}\". This is still the same project but at a possibly new location; take this into account when working with any files from now on.</system-reminder>")
}

fn toast_error(toast: &mut ToastState, title: &str, err: &Value) {
    toast.show(ToastInput {
        title: Some(title.to_string()),
        message: error_message_value(err),
        variant: Some(ToastVariant::Error),
        duration_ms: None,
    });
}

/// Mirrors the adapter load (failure toast verbatim).
pub async fn load_workspace_adapters(
    client: &Arc<dyn SdkClient>,
    directory: &str,
    toast: &mut ToastState,
) -> Option<Vec<Value>> {
    match client
        .call(
            "experimental.workspace.adapter.list",
            serde_json::json!({ "directory": directory }),
        )
        .await
    {
        Ok(response) => response.get("data").and_then(|d| d.as_array()).cloned(),
        Err(err) => {
            toast.show(ToastInput {
                title: Some("Failed to load workspace adapters".to_string()),
                message: err,
                variant: Some(ToastVariant::Error),
                duration_ms: None,
            });
            None
        }
    }
}

/// Dialog actions available to the workspace flows.
pub struct WorkspaceFlow<'a> {
    pub stack: &'a mut DialogStack,
    pub client: Arc<dyn SdkClient>,
    pub toast: &'a mut ToastState,
}

/// Mirrors `DialogWorkspaceSelect` options (adapters + None + recent +
/// view-all escape hatch).
pub fn workspace_select_options(
    adapters: &[Value],
    recent: &[Value],
    has_more: bool,
) -> Vec<(SelectOption, WorkspaceSelectValue)> {
    let mut options: Vec<(SelectOption, WorkspaceSelectValue)> = adapters
        .iter()
        .map(|adapter| {
            let name = adapter.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let workspace_type = adapter.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string();
            (
                SelectOption {
                    title: name.clone(),
                    description: adapter.get("description").and_then(|v| v.as_str()).map(str::to_string),
                    category: Some("New workspace".to_string()),
                    value: serde_json::json!({ "kind": "new", "workspaceType": workspace_type, "workspaceName": name }),
                    ..SelectOption::default()
                },
                WorkspaceSelectValue::New,
            )
        })
        .collect();
    options.push((
        SelectOption {
            title: "None".to_string(),
            description: Some("Use the local project".to_string()),
            category: Some("Choose workspace".to_string()),
            value: serde_json::json!({ "kind": "none" }),
            ..SelectOption::default()
        },
        WorkspaceSelectValue::None,
    ));
    options.extend(recent.iter().map(|workspace| {
        let name = workspace
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let workspace_type = workspace
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        (
            SelectOption {
                title: name.clone(),
                description: Some(format!("({workspace_type})")),
                category: Some("Choose workspace".to_string()),
                value: serde_json::json!({
                    "kind": "existing",
                    "workspaceID": workspace.get("id"),
                    "workspaceType": workspace_type,
                    "workspaceName": name,
                }),
                ..SelectOption::default()
            },
            WorkspaceSelectValue::ExistingList,
        )
    }));
    if has_more {
        options.push((
            SelectOption {
                title: "View all workspaces".to_string(),
                description: Some("Choose from all workspaces".to_string()),
                category: Some("Choose workspace".to_string()),
                value: serde_json::json!({ "kind": "existing-list" }),
                ..SelectOption::default()
            },
            WorkspaceSelectValue::ExistingList,
        ));
    }
    options
}

/// Build the `Warp` select state.
pub fn warp_state(
    pairs: Vec<(SelectOption, WorkspaceSelectValue)>,
) -> (SelectState, Vec<WorkspaceSelectValue>) {
    let (options, kinds): (Vec<SelectOption>, Vec<WorkspaceSelectValue>) =
        pairs.into_iter().unzip();
    let mut state = SelectState::new("Warp", options);
    state.skip_filter = true;
    state.render_filter = false;
    (state, kinds)
}

/// Build the `Existing Workspace` select state (connected, minus omitted).
pub fn existing_workspace_state(
    workspaces: &[Value],
    omit_workspace_id: Option<&str>,
) -> SelectState {
    let options = workspaces
        .iter()
        .filter(|w| {
            let id = w.get("id").and_then(|v| v.as_str()).unwrap_or("");
            Some(id) != omit_workspace_id
        })
        .map(|workspace| {
            let name = workspace
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let workspace_type = workspace
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            SelectOption {
                title: name,
                description: Some(format!("({workspace_type})")),
                value: workspace.clone(),
                ..SelectOption::default()
            }
        })
        .collect();
    SelectState::new("Existing Workspace", options)
}

/// Mirrors `confirmWorkspaceFileChanges` — `None` on dismiss, else the bool.
pub async fn confirm_workspace_file_changes(
    stack: &mut DialogStack,
    client: &Arc<dyn SdkClient>,
    source_workspace_id: Option<&str>,
) -> Option<bool> {
    let status = client
        .call(
            "vcs.status",
            match source_workspace_id {
                Some(id) => serde_json::json!({ "workspace": id }),
                None => serde_json::json!({}),
            },
        )
        .await
        .ok();
    let files = status
        .as_ref()
        .and_then(|s| s.get("data"))
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default();
    if files.is_empty() {
        return Some(false);
    }
    match show_file_changes(stack, files, None, None).await {
        Ok(Some(FileChangesChoice::Yes)) => Some(true),
        Ok(Some(FileChangesChoice::No)) => Some(false),
        _ => None,
    }
}

/// Mirrors `warpWorkspaceSession` (returns the warped flag).
#[allow(clippy::too_many_arguments)]
pub async fn warp_workspace_session(
    stack: &mut DialogStack,
    client: &Arc<dyn SdkClient>,
    sync: &mut SyncStore,
    project: &mut ProjectStore,
    toast: &mut ToastState,
    source_workspace_id: Option<String>,
    workspace_id: Option<String>,
    session_id: &str,
    copy_changes: bool,
    instance_directory: &str,
    sync_directory: &str,
) -> bool {
    let result = client
        .call(
            "experimental.workspace.warp",
            serde_json::json!({ "id": workspace_id, "sessionID": session_id, "copyChanges": copy_changes }),
        )
        .await;
    let data = match result {
        Ok(response) => response.get("data").cloned(),
        Err(err) => {
            // The transport surfaces error names in the message; a
            // `VcsApplyError` keeps the session unwarped with an alert
            // (mirrors the `result.error.name` branch).
            if err.contains("VcsApplyError") {
                let _ = show_alert(
                    stack,
                    "Unable to Warp Session",
                    "Unable to apply file changes to this workspace. It has existing changes that conflict or is based off a different branch. Session has not been warped.",
                )
                .await;
                return false;
            }
            toast_error(toast, "Failed to warp session", &Value::String(err));
            return false;
        }
    };
    if data.is_none() {
        toast_error(
            toast,
            "Failed to warp session",
            &Value::String("no response".to_string()),
        );
        return false;
    }
    project.workspace_set(workspace_id);
    let _ = sync.bootstrap(project, false).await;
    let dir = if instance_directory.is_empty() {
        sync_directory
    } else {
        instance_directory
    };
    if !dir.is_empty() {
        let _ = client
            .call(
                "session.promptAsync",
                serde_json::json!({
                    "sessionID": session_id,
                    "workspace": source_workspace_id,
                    "noReply": true,
                    "parts": [{ "type": "text", "text": warp_reminder_text(dir), "synthetic": true }],
                }),
            )
            .await;
    }
    let _ = project.sync_workspace().await;
    let _ = sync.session_refresh(project).await;
    stack.clear();
    true
}
