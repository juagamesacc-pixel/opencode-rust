// source: packages/tui/src/component/prompt/move.tsx (205 lines, v1.18.30)
// 1:1 port — move-session flow: project-copy creation with progress
// states, existing-session move with the file-changes gate, reminder
// prompt, destination pending tracking, and the creating-dots timer.

#![allow(dead_code)]

use std::sync::Arc;
use std::time::{Duration, Instant};

use super::super::dialog_move_session::MoveSessionSelection;
use crate::context::sdk::SdkClient;
use crate::context::sync::SyncStore;
use crate::routes::home::session_destination::HomeSessionDestinationState;
use crate::ui::dialog::DialogStack;
use crate::ui::dialog_workspace_file_changes::{show_file_changes, FileChangesChoice};
use crate::ui::toast::{ToastInput, ToastState};
use crate::util::selection::ToastVariant;

/// Mirrors `moveReminderText`.
pub fn move_reminder_text(directory: &str) -> String {
    format!("<system-reminder>The user has changed the current working directory to \"{directory}\". This is still the same project but at a possibly new location; take this into account when working with any files from now on.</system-reminder>")
}

/// Move flow state (mirrors the hook's signals).
pub struct PromptMove {
    pub creating: bool,
    pub creating_dots: u32,
    pub progress: Option<String>,
    dots_deadline: Option<Instant>,
}

impl PromptMove {
    pub fn new() -> Self {
        Self {
            creating: false,
            creating_dots: 3,
            progress: None,
            dots_deadline: None,
        }
    }

    /// Dots timer driver (mirrors the 1s interval effect).
    pub fn poll(&mut self) {
        if !self.creating {
            self.creating_dots = 3;
            self.dots_deadline = None;
            return;
        }
        let now = Instant::now();
        match self.dots_deadline {
            Some(deadline) if now >= deadline => {
                self.creating_dots = (self.creating_dots % 3) + 1;
                self.dots_deadline = Some(now + Duration::from_secs(1));
            }
            None => {
                self.dots_deadline = Some(now + Duration::from_secs(1));
            }
            _ => {}
        }
    }

    /// Mirrors `create` — generate a worktree copy, returning its directory.
    /// `home_clear` runs on failure (mirrors `homeDestination?.clear()`).
    pub async fn create(
        &mut self,
        client: &Arc<dyn SdkClient>,
        toast: &mut ToastState,
        project_id: &str,
        sdk_directory: Option<&str>,
        worktree_base: &str,
        context: Option<&str>,
        home_clear: &mut dyn FnMut(),
    ) -> Option<String> {
        self.creating = true;
        self.progress = Some("Creating copy".to_string());
        let generated = client
            .call(
                "experimental.projectCopy.generateName",
                serde_json::json!({ "projectID": project_id, "context": context }),
            )
            .await;
        let name = match generated {
            Ok(response) => response
                .get("data")
                .and_then(|d| d.get("name"))
                .and_then(|v| v.as_str())
                .map(str::to_string),
            Err(err) => {
                self.fail(toast, home_clear, &err);
                return None;
            }
        };
        let directory = format!(
            "{}/{}",
            worktree_base.trim_end_matches('/'),
            project_id.get(..6).unwrap_or(project_id)
        );
        let result = client
            .call(
                "v2.projectCopy.create",
                serde_json::json!({
                    "projectID": project_id,
                    "location": { "directory": sdk_directory },
                    "strategy": "git_worktree",
                    "directory": directory,
                    "name": name,
                }),
            )
            .await;
        let target = match result {
            Ok(response) => response
                .get("data")
                .and_then(|d| d.get("directory"))
                .and_then(|v| v.as_str())
                .map(str::to_string),
            Err(err) => {
                self.fail(toast, home_clear, &err);
                return None;
            }
        };
        let Some(target) = target else {
            self.fail(toast, home_clear, "No project copy directory returned");
            return None;
        };
        // Bootstrap the location route before moving on (mirrors path.get).
        if client
            .call("path.get", serde_json::json!({ "directory": target }))
            .await
            .is_err()
        {
            self.fail(toast, home_clear, "No project copy directory returned");
            return None;
        }
        self.progress = Some("Creating session".to_string());
        Some(target)
    }

    fn fail(&mut self, toast: &mut ToastState, home_clear: &mut dyn FnMut(), err: &str) {
        home_clear();
        self.progress = None;
        self.creating = false;
        toast.show(ToastInput {
            title: Some("Creating workspace failed".to_string()),
            message: err.to_string(),
            variant: Some(ToastVariant::Error),
            duration_ms: None,
        });
    }

    /// Mirrors `sessionContext` — title plus the last 6 messages' texts.
    pub fn session_context(sync: &SyncStore, session_id: &str) -> Option<String> {
        let mut lines: Vec<String> = Vec::new();
        if let Some(title) = sync
            .session_get(session_id)
            .and_then(|s| s.get("title"))
            .and_then(|v| v.as_str())
        {
            lines.push(title.to_string());
        }
        if let Some(messages) = sync.message.get(session_id) {
            for message in messages.iter().take(6) {
                let role = message.get("role").and_then(|v| v.as_str()).unwrap_or("");
                let id = message.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let texts: Vec<&str> = sync
                    .part
                    .get(id)
                    .map(|parts| {
                        parts
                            .iter()
                            .filter(|part| {
                                part.get("type").and_then(|v| v.as_str()) == Some("text")
                            })
                            .filter_map(|part| part.get("text").and_then(|v| v.as_str()))
                            .collect()
                    })
                    .unwrap_or_default();
                lines.push(format!("{role}: {}", texts.join(" ")));
            }
        }
        let joined = lines
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        if joined.is_empty() {
            None
        } else {
            Some(joined)
        }
    }

    /// Mirrors `moveExistingSession`.
    #[allow(clippy::too_many_arguments)]
    pub async fn move_existing_session(
        &mut self,
        stack: &mut DialogStack,
        client: &Arc<dyn SdkClient>,
        toast: &mut ToastState,
        sync: &SyncStore,
        session_id: &str,
        session_directory: Option<&str>,
        selection: &MoveSessionSelection,
        home_destination: &mut HomeSessionDestinationState,
        worktree_base: &str,
        sdk_directory: Option<&str>,
        project_id: &str,
    ) {
        let status = client
            .call(
                "vcs.status",
                serde_json::json!({ "directory": session_directory }),
            )
            .await
            .ok();
        let files = status
            .as_ref()
            .and_then(|s| s.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        let choice = if files.is_empty() {
            Some(FileChangesChoice::No)
        } else {
            show_file_changes(stack, files, None, None)
                .await
                .ok()
                .flatten()
        };
        let Some(choice) = choice else { return };
        stack.clear();
        let directory = match selection {
            MoveSessionSelection::New => {
                let context = Self::session_context(sync, session_id);
                let mut home_noop = || {};
                self.create(
                    client,
                    toast,
                    project_id,
                    sdk_directory,
                    worktree_base,
                    context.as_deref(),
                    &mut home_noop,
                )
                .await
            }
            MoveSessionSelection::Directory { directory, .. } => Some(directory.clone()),
        };
        let Some(directory) = directory else {
            self.progress = None;
            stack.clear();
            return;
        };
        self.progress = Some("Moving session".to_string());
        let result = client
            .call(
                "experimental.controlPlane.moveSession",
                serde_json::json!({
                    "sessionID": session_id,
                    "destination": { "directory": directory },
                    "moveChanges": choice == FileChangesChoice::Yes,
                }),
            )
            .await;
        if let Err(err) = result {
            toast.show(ToastInput {
                title: None,
                message: err,
                variant: Some(ToastVariant::Error),
                duration_ms: None,
            });
            stack.clear();
        } else {
            let _ = client
                .call(
                    "session.promptAsync",
                    serde_json::json!({
                        "sessionID": session_id,
                        "directory": directory,
                        "noReply": true,
                        "parts": [{ "type": "text", "text": move_reminder_text(&directory), "synthetic": true }],
                    }),
                )
                .await;
            stack.clear();
        }
        self.progress = None;
        self.creating = false;
        let _ = home_destination;
    }

    /// Mirrors the `pending` memo — true while the destination provider is
    /// present (the memo itself is always truthy, verbatim).
    pub fn pending(has_destination_provider: bool) -> bool {
        has_destination_provider
    }

    pub fn pending_new(
        home_destination: &HomeSessionDestinationState,
        sync_directory: &str,
        cwd: &str,
    ) -> bool {
        use crate::routes::home::session_destination::HomeSessionDestination;
        matches!(
            home_destination.destination(sync_directory, cwd),
            HomeSessionDestination::New
        )
    }

    /// Mirrors `getDirectory` (directory selections only).
    pub fn get_directory(home_destination: &HomeSessionDestinationState) -> Option<String> {
        match home_destination.selected() {
            Some(HomeSessionDestination::Directory { directory, .. }) => Some(directory.clone()),
            _ => None,
        }
    }

    pub fn start_submit(&mut self) {
        if self.progress.is_some() {
            self.progress = Some("Submitting prompt".to_string());
        }
    }

    pub fn finish_submit(&mut self, home_destination: &mut HomeSessionDestinationState) {
        home_destination.clear();
        self.progress = None;
        self.creating = false;
    }
}

impl Default for PromptMove {
    fn default() -> Self {
        Self::new()
    }
}
