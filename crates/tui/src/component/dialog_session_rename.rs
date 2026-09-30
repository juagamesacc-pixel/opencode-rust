// source: packages/tui/src/component/dialog-session-rename.tsx (31 lines, v1.18.30)
// 1:1 port — prompt wrapper: current title as value, update + clear on
// confirm, clear on cancel.

#![allow(dead_code)]

use crate::ui::dialog_prompt::PromptProps;

/// Build the rename prompt props (title `Rename Session`, verbatim).
pub fn rename_prompt_props(current_title: Option<&str>) -> PromptProps {
    PromptProps {
        title: "Rename Session".to_string(),
        description: Vec::new(),
        placeholder: None,
        value: current_title.map(str::to_string),
        busy: false,
        busy_text: None,
        submit_hint: None,
    }
}

/// Session-update params for the rename confirm (mirrors
/// `session.update({ sessionID, title })`).
pub fn rename_update_params(session_id: &str, title: &str) -> serde_json::Value {
    serde_json::json!({ "sessionID": session_id, "title": title })
}
