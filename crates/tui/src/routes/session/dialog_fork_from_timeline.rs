// source: packages/tui/src/routes/session/dialog-fork-from-timeline.tsx (76 lines, v1.18.30)
// 1:1 port — `Full session` head option plus the timeline rows; fork
// params carry the rebuilt prompt (ids stripped, verbatim).

#![allow(dead_code)]

use serde_json::Value;

use super::dialog_message::fork_prompt_from_parts;
use super::dialog_timeline::timeline_options;
use crate::prompt::history::PromptInfo;
use crate::ui::dialog_select::{SelectOption, SelectState};

/// Full-session head option (value null, title verbatim).
pub fn full_session_option() -> SelectOption {
    SelectOption {
        title: "Full session".to_string(),
        value: Value::Null,
        ..SelectOption::default()
    }
}

/// Build the fork select state (title `Fork session`, large size).
pub fn fork_state(messages: &[Value], parts_of: &dyn Fn(&str) -> Vec<Value>) -> SelectState {
    let mut options = vec![full_session_option()];
    options.extend(timeline_options(messages, parts_of));
    SelectState::new("Fork session", options)
}

/// Fork params for a timeline row (message-scoped fork + rebuilt prompt).
pub fn fork_params(
    session_id: &str,
    message_id: Option<&str>,
    parts: &[Value],
) -> (Value, Option<PromptInfo>) {
    let params = match message_id {
        Some(id) => serde_json::json!({ "sessionID": session_id, "messageID": id }),
        None => serde_json::json!({ "sessionID": session_id }),
    };
    let prompt = match message_id {
        Some(_) => Some(fork_prompt_from_parts(parts)),
        None => None,
    };
    (params, prompt)
}
