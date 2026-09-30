// source: packages/tui/src/component/dialog-agent.tsx (31 lines, v1.18.30)
// 1:1 port — agent options (native marker) over `SelectState`; selection
// applies the agent and clears the dialog.

#![allow(dead_code)]

use crate::context::local::LocalContext;
use crate::context::sync::SyncStore;
use crate::ui::dialog_select::{SelectOption, SelectState};

/// Build the agent select state (mirrors the options memo).
pub fn agent_state(local: &LocalContext, sync: &SyncStore) -> SelectState {
    let options = local
        .agents(sync)
        .iter()
        .map(|item| SelectOption {
            value: serde_json::Value::String(
                item.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            ),
            title: item
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            description: Some(
                if item.get("native").and_then(|v| v.as_bool()) == Some(true) {
                    "native".to_string()
                } else {
                    item.get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                },
            ),
            ..SelectOption::default()
        })
        .collect();
    let mut state = SelectState::new("Select agent", options);
    state.current = local
        .agent_current(sync)
        .and_then(|a| a.get("name"))
        .cloned();
    state
}
