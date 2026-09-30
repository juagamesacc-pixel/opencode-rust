// source: packages/tui/src/component/dialog-workspace-list.tsx (112 lines, v1.18.30)
// 1:1 port — name-sorted options with deleting/removing titles, status
// dot gutters, directory details on expand, two-step delete, and the
// current-workspace home navigation after removal.

#![allow(dead_code)]

use serde_json::Value;

use crate::ui::dialog_select::{SelectOption, SelectState};

/// Workspace list dialog state.
#[derive(Debug, Default)]
pub struct WorkspaceListState {
    pub deleting: Option<String>,
    pub removing: Option<String>,
    pub expanded: Vec<String>,
}

impl WorkspaceListState {
    /// Build options (name-sorted; titles reflect deleting/removing).
    pub fn build_options(
        &self,
        workspaces: &[Value],
        connected: impl Fn(&str) -> bool,
    ) -> Vec<SelectOption> {
        let mut sorted: Vec<&Value> = workspaces.iter().collect();
        sorted.sort_by(|a, b| {
            a.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("name").and_then(|v| v.as_str()).unwrap_or(""))
        });
        sorted
            .into_iter()
            .map(|workspace| {
                let id = workspace
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let name = workspace
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let title = if self.removing.as_deref() == Some(id.as_str()) {
                    "Deleting…".to_string()
                } else if self.deleting.as_deref() == Some(id.as_str()) {
                    format!("Delete {name}? Press delete again")
                } else {
                    name
                };
                let mut option = SelectOption {
                    title,
                    value: serde_json::json!({ "workspace": workspace }),
                    footer: workspace
                        .get("type")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    ..SelectOption::default()
                };
                if self.expanded.iter().any(|expanded| expanded == &id) {
                    if let Some(directory) = workspace.get("directory").and_then(|v| v.as_str()) {
                        option.details = vec![directory.to_string()];
                    }
                }
                option.gutter =
                    Some(if connected(&id) { "connected" } else { "error" }.to_string());
                option
            })
            .collect()
    }

    pub fn toggle_expanded(&mut self, workspace_id: &str) {
        if let Some(index) = self.expanded.iter().position(|id| id == workspace_id) {
            self.expanded.remove(index);
        } else {
            self.expanded.push(workspace_id.to_string());
        }
    }

    /// First delete press arms, second press confirms (mirrors `remove`
    /// up to the SDK call; the caller performs it when this returns true).
    pub fn delete_press(&mut self, workspace_id: &str) -> bool {
        if self.removing.is_some() {
            return false;
        }
        if self.deleting.as_deref() != Some(workspace_id) {
            self.deleting = Some(workspace_id.to_string());
            return false;
        }
        self.deleting = None;
        self.removing = Some(workspace_id.to_string());
        true
    }

    pub fn delete_done(&mut self) {
        self.removing = None;
    }

    pub fn select_state(
        &self,
        workspaces: &[Value],
        connected: impl Fn(&str) -> bool,
    ) -> SelectState {
        SelectState::new("Workspaces", self.build_options(workspaces, connected))
    }
}
