// source: packages/tui/src/component/prompt/workspace.tsx (137 lines, v1.18.30)
// 1:1 port — workspace prompt flow: creation with the verbatim failure
// toasts, warp with the file-changes gate, 4s warped notice, dots timer,
// and the label memo.

#![allow(dead_code)]

use std::sync::Arc;
use std::time::{Duration, Instant};

use super::super::dialog_workspace_create::{
    confirm_workspace_file_changes, warp_reminder_text, warp_workspace_session, WorkspaceSelection,
};
use crate::context::project::ProjectStore;
use crate::context::sdk::SdkClient;
use crate::context::sync::SyncStore;
use crate::ui::dialog::DialogStack;
use crate::ui::toast::{ToastInput, ToastState};
use crate::util::error::error_message_value;
use crate::util::selection::ToastVariant;
use serde_json::Value;

/// Notice duration verbatim (4000ms).
pub const WARP_NOTICE_MS: u64 = 4000;

/// Mirrors the label memo shapes.
#[derive(Debug, Clone)]
pub enum WorkspaceLabel {
    New {
        workspace_type: String,
    },
    Existing {
        workspace_type: String,
        workspace_name: String,
        connected: bool,
    },
}

/// Workspace prompt flow state.
pub struct PromptWorkspace {
    pub selection: Option<WorkspaceSelection>,
    pub creating: bool,
    pub creating_dots: u32,
    pub notice: Option<String>,
    notice_deadline: Option<Instant>,
    dots_deadline: Option<Instant>,
}

impl PromptWorkspace {
    pub fn new() -> Self {
        Self {
            selection: None,
            creating: false,
            creating_dots: 3,
            notice: None,
            notice_deadline: None,
            dots_deadline: None,
        }
    }

    /// Dots timer + notice expiry driver (mirrors the interval + timeout).
    pub fn poll(&mut self) {
        let now = Instant::now();
        if !self.creating {
            self.creating_dots = 3;
            self.dots_deadline = None;
        } else if self.dots_deadline.map(|d| now >= d).unwrap_or(true) {
            self.creating_dots = (self.creating_dots % 3) + 1;
            self.dots_deadline = Some(now + Duration::from_secs(1));
        }
        if self.notice_deadline.map(|d| now >= d).unwrap_or(false) {
            self.notice = None;
            self.notice_deadline = None;
        }
    }

    /// Mirrors `create` — returns the created workspace on success.
    pub async fn create(
        &mut self,
        client: &Arc<dyn SdkClient>,
        project: &mut ProjectStore,
        toast: &mut ToastState,
        workspace_type: &str,
    ) -> Option<Value> {
        self.creating = true;
        let result = client
            .call(
                "experimental.workspace.create",
                serde_json::json!({ "type": workspace_type, "branch": null }),
            )
            .await;
        let data = match result {
            Ok(response) => response.get("data").cloned(),
            Err(err) => {
                self.selection = None;
                self.creating = false;
                toast.show(ToastInput {
                    title: Some("Creating workspace failed".to_string()),
                    message: err,
                    variant: Some(ToastVariant::Error),
                    duration_ms: None,
                });
                return None;
            }
        };
        let Some(workspace) = data else {
            self.selection = None;
            self.creating = false;
            toast.show(ToastInput {
                title: Some("Creating workspace failed".to_string()),
                message: error_message_value(&Value::String("no response".to_string())),
                variant: Some(ToastVariant::Error),
                duration_ms: None,
            });
            return None;
        };
        let _ = project.sync_workspace().await;
        self.selection = Some(WorkspaceSelection::Existing {
            workspace_id: workspace
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            workspace_type: workspace
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            workspace_name: workspace
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        });
        self.creating = false;
        Some(workspace)
    }

    /// Mirrors `warp`.
    #[allow(clippy::too_many_arguments)]
    pub async fn warp(
        &mut self,
        stack: &mut DialogStack,
        client: &Arc<dyn SdkClient>,
        sync: &mut SyncStore,
        project: &mut ProjectStore,
        toast: &mut ToastState,
        session_id: Option<&str>,
        instance_directory: &str,
        sync_directory: &str,
        selection: WorkspaceSelection,
    ) {
        let Some(session_id) = session_id else {
            self.selection = Some(selection.clone());
            stack.clear();
            if matches!(selection, WorkspaceSelection::New { .. }) {
                if let WorkspaceSelection::New { workspace_type, .. } = selection {
                    let _ = self.create(client, project, toast, &workspace_type).await;
                }
            }
            return;
        };
        let source_workspace_id = project.workspace_current().map(str::to_string);
        let copy_changes =
            match confirm_workspace_file_changes(stack, client, source_workspace_id.as_deref())
                .await
            {
                Some(copy) => copy,
                None => return,
            };
        self.selection = Some(selection.clone());
        stack.clear();
        let (workspace_id, workspace_name) = match &selection {
            WorkspaceSelection::None => (None, "local project".to_string()),
            WorkspaceSelection::Existing {
                workspace_id,
                workspace_name,
                ..
            } => (Some(workspace_id.clone()), workspace_name.clone()),
            WorkspaceSelection::New { workspace_type, .. } => {
                match self.create(client, project, toast, workspace_type).await {
                    Some(workspace) => (
                        workspace
                            .get("id")
                            .and_then(|v| v.as_str())
                            .map(str::to_string),
                        workspace
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    ),
                    None => return,
                }
            }
        };
        let warped = warp_workspace_session(
            stack,
            client,
            sync,
            project,
            toast,
            source_workspace_id,
            workspace_id,
            session_id,
            copy_changes,
            instance_directory,
            sync_directory,
        )
        .await;
        if warped {
            self.show_notice(&workspace_name);
        }
        let _ = warp_reminder_text;
    }

    fn show_notice(&mut self, name: &str) {
        self.notice = Some(format!("Warped to {name}"));
        self.notice_deadline = Some(Instant::now() + Duration::from_millis(WARP_NOTICE_MS));
    }

    pub fn clear_notice(&mut self) {
        self.notice = None;
        self.notice_deadline = None;
    }

    /// Mirrors the `label` memo.
    pub fn label(&self, session_id: Option<&str>) -> Option<WorkspaceLabel> {
        let selected = self.selection.as_ref()?;
        if matches!(selected, WorkspaceSelection::None) {
            return None;
        }
        if session_id.is_some() && !self.creating {
            return None;
        }
        match selected {
            WorkspaceSelection::New { workspace_type, .. } => Some(WorkspaceLabel::New {
                workspace_type: workspace_type.clone(),
            }),
            WorkspaceSelection::Existing {
                workspace_type,
                workspace_name,
                ..
            } => Some(WorkspaceLabel::Existing {
                workspace_type: workspace_type.clone(),
                workspace_name: workspace_name.clone(),
                connected: true,
            }),
            WorkspaceSelection::None => None,
        }
    }
}

impl Default for PromptWorkspace {
    fn default() -> Self {
        Self::new()
    }
}
