// source: packages/tui/src/routes/session/dialog-timeline.tsx (47 lines, v1.18.30)
// 1:1 port — user-message timeline (first non-synthetic non-ignored text
// part, newlines flattened, time footer), newest first.

#![allow(dead_code)]

use serde_json::Value;

use crate::ui::dialog_select::{SelectOption, SelectState};
use crate::util::locale::time;

/// Build timeline options from session messages + parts.
pub fn timeline_options(
    messages: &[Value],
    parts_of: &dyn Fn(&str) -> Vec<Value>,
) -> Vec<SelectOption> {
    let mut options: Vec<SelectOption> = Vec::new();
    for message in messages {
        if message.get("role").and_then(|v| v.as_str()) != Some("user") {
            continue;
        }
        let id = message
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let created = message
            .get("time")
            .and_then(|t| t.get("created"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let text = parts_of(&id).iter().find_map(|part| {
            if part.get("type").and_then(|v| v.as_str()) != Some("text") {
                return None;
            }
            if part.get("synthetic").and_then(|v| v.as_bool()) == Some(true) {
                return None;
            }
            if part.get("ignored").and_then(|v| v.as_bool()) == Some(true) {
                return None;
            }
            part.get("text")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        });
        let Some(text) = text else { continue };
        options.push(SelectOption {
            title: text.replace('\n', " "),
            value: Value::String(id),
            footer: Some(time(created)),
            ..SelectOption::default()
        });
    }
    options.reverse();
    options
}

/// Build the timeline select state (title `Timeline`, large size).
pub fn timeline_state(messages: &[Value], parts_of: &dyn Fn(&str) -> Vec<Value>) -> SelectState {
    SelectState::new("Timeline", timeline_options(messages, parts_of))
}
