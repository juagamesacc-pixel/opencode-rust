// source: packages/tui/src/routes/session/dialog-subagent.tsx (26 lines, v1.18.30)
// 1:1 port — single `Open` action navigating to the subagent session.

#![allow(dead_code)]

use serde_json::Value;

use crate::ui::dialog_select::{SelectOption, SelectState};

/// Build the subagent-actions state (title `Subagent Actions`).
pub fn subagent_actions_state(session_id: &str) -> SelectState {
    SelectState::new(
        "Subagent Actions",
        vec![SelectOption {
            title: "Open".to_string(),
            value: Value::String(session_id.to_string()),
            description: Some("the subagent's session".to_string()),
            ..SelectOption::default()
        }],
    )
}
