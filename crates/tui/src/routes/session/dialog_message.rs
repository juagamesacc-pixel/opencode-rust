// source: packages/tui/src/routes/session/dialog-message.tsx (109 lines, v1.18.30)
// 1:1 port — message actions (Revert/Copy/Fork) with verbatim copy;
// revert rebuilds the prompt from non-synthetic text + stripped file
// parts; fork navigates to the new session with the same rebuilt prompt.

#![allow(dead_code)]

use serde_json::Value;

use crate::prompt::history::PromptInfo;
use crate::prompt::part::strip_prompt_part_ids;
use crate::ui::dialog_select::{SelectOption, SelectState};

/// Option values verbatim.
pub const MESSAGE_ACTION_REVERT: &str = "session.revert";
pub const MESSAGE_ACTION_COPY: &str = "message.copy";
pub const MESSAGE_ACTION_FORK: &str = "session.fork";

/// Build the message-actions select state (title `Message Actions`).
pub fn message_actions_state() -> SelectState {
    SelectState::new(
        "Message Actions",
        vec![
            SelectOption {
                title: "Revert".to_string(),
                value: Value::String(MESSAGE_ACTION_REVERT.to_string()),
                description: Some("undo messages and file changes".to_string()),
                ..SelectOption::default()
            },
            SelectOption {
                title: "Copy".to_string(),
                value: Value::String(MESSAGE_ACTION_COPY.to_string()),
                description: Some("message text to clipboard".to_string()),
                ..SelectOption::default()
            },
            SelectOption {
                title: "Fork".to_string(),
                value: Value::String(MESSAGE_ACTION_FORK.to_string()),
                description: Some("create a new session".to_string()),
                ..SelectOption::default()
            },
        ],
    )
}

/// Rebuild the prompt from message parts (mirrors the revert reducer:
/// non-synthetic text concatenated, file parts stripped of ids).
pub fn rebuild_prompt_from_parts(parts: &[Value]) -> PromptInfo {
    let mut prompt = PromptInfo {
        input: String::new(),
        mode: None,
        parts: Vec::new(),
    };
    for part in parts {
        let kind = part.get("type").and_then(|v| v.as_str()).unwrap_or("");
        if kind == "text" && part.get("synthetic").and_then(|v| v.as_bool()) != Some(true) {
            prompt.input += part.get("text").and_then(|v| v.as_str()).unwrap_or("");
        }
        if kind == "file" {
            prompt.parts.push(strip_prompt_part_ids(part));
        }
    }
    prompt
}

/// Copy text builder (non-synthetic text concatenated, verbatim).
pub fn copy_text_from_parts(parts: &[Value]) -> String {
    let mut text = String::new();
    for part in parts {
        if part.get("type").and_then(|v| v.as_str()) == Some("text")
            && part.get("synthetic").and_then(|v| v.as_bool()) != Some(true)
        {
            text += part.get("text").and_then(|v| v.as_str()).unwrap_or("");
        }
    }
    text
}

/// Fork prompt builder (same as revert but file parts keep their ids —
/// mirrors the fork reducer verbatim).
pub fn fork_prompt_from_parts(parts: &[Value]) -> PromptInfo {
    let mut prompt = PromptInfo {
        input: String::new(),
        mode: None,
        parts: Vec::new(),
    };
    for part in parts {
        let kind = part.get("type").and_then(|v| v.as_str()).unwrap_or("");
        if kind == "text" && part.get("synthetic").and_then(|v| v.as_bool()) != Some(true) {
            prompt.input += part.get("text").and_then(|v| v.as_str()).unwrap_or("");
        }
        if kind == "file" {
            prompt.parts.push(part.clone());
        }
    }
    prompt
}
