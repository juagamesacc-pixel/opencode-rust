// source: packages/tui/src/component/dialog-skill.tsx (70 lines, v1.18.30)
// 1:1 port — name-padded skill options (whitespace-collapsed
// descriptions), error view copy (`Could not load skills`).

#![allow(dead_code)]

use serde_json::Value;

use crate::ui::dialog_select::{SelectOption, SelectState};

/// Build skill options (titles padded to the longest name, verbatim).
pub fn skill_options(skills: &[Value]) -> Vec<SelectOption> {
    let max_width = skills
        .iter()
        .filter_map(|skill| skill.get("name"))
        .filter_map(|name| name.as_str())
        .map(|name| name.chars().count())
        .max()
        .unwrap_or(0);
    skills
        .iter()
        .map(|skill| {
            let name = skill.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let description =
                skill
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|description| {
                        description
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string()
                    });
            SelectOption {
                title: format!("{name:<max_width$}", max_width = max_width),
                description,
                category: Some("Skills".to_string()),
                value: Value::String(name.to_string()),
                ..SelectOption::default()
            }
        })
        .collect()
}

/// Build the skills select state (placeholder `Search skills…`).
pub fn skill_state(skills: &[Value]) -> SelectState {
    let mut state = SelectState::new("Skills", skill_options(skills));
    state.placeholder = "Search skills…".to_string();
    state
}

/// Error view copy verbatim.
pub const SKILL_LOAD_ERROR_TITLE: &str = "Could not load skills";
