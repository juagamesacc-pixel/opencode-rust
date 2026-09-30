// source: packages/tui/src/component/dialog-stash.tsx (87 lines, v1.18.30)
// 1:1 port — reversed entries, first-line preview (50 chars), relative
// times, `~N lines` footers, two-step delete with error-color rows.

#![allow(dead_code)]

use crate::prompt::stash::StashEntry;
use crate::ui::dialog_select::{
    SelectAction, SelectDisabled, SelectOption, SelectSide, SelectState,
};
use crate::util::locale::{datetime, truncate};

/// Preview length verbatim (50).
pub const STASH_PREVIEW_LEN: usize = 50;

/// Mirrors `getRelativeTime`.
pub fn relative_time(timestamp_ms: i64, now_ms: i64) -> String {
    let diff = now_ms.saturating_sub(timestamp_ms);
    let seconds = diff / 1000;
    if seconds < 60 {
        return "just now".to_string();
    }
    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{minutes}m ago");
    }
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{hours}h ago");
    }
    let days = hours / 24;
    if days < 7 {
        return format!("{days}d ago");
    }
    datetime(timestamp_ms)
}

/// Mirrors `getStashPreview` (first line, trimmed, truncated).
pub fn stash_preview(input: &str) -> String {
    let first = input.split('\n').next().unwrap_or("").trim();
    truncate(first, STASH_PREVIEW_LEN)
}

/// Build stash options (most-recent first, delete-confirm rows).
pub fn stash_options(
    entries: &[StashEntry],
    to_delete: Option<usize>,
    now_ms: i64,
    delete_hint: &str,
) -> Vec<SelectOption> {
    entries
        .iter()
        .enumerate()
        .rev()
        .map(|(index, entry)| {
            let deleting = Some(index) == to_delete;
            let line_count = entry.input.matches('\n').count() + 1;
            SelectOption {
                title: if deleting {
                    format!("Press {delete_hint} again to confirm")
                } else {
                    stash_preview(&entry.input)
                },
                bg: if deleting {
                    Some(crate::theme::Rgba::from_hex("#e06c75"))
                } else {
                    None
                },
                value: serde_json::Value::Number(index.into()),
                description: Some(relative_time(entry.timestamp, now_ms)),
                footer: if line_count > 1 {
                    Some(format!("~{line_count} lines"))
                } else {
                    None
                },
                ..SelectOption::default()
            }
        })
        .collect()
}

/// Build the stash select state (title `Stash` + delete action).
pub fn stash_state(
    entries: &[StashEntry],
    to_delete: Option<usize>,
    now_ms: i64,
    delete_hint: &str,
) -> SelectState {
    let mut state = SelectState::new(
        "Stash",
        stash_options(entries, to_delete, now_ms, delete_hint),
    );
    state.actions = vec![SelectAction {
        command: "stash.delete".to_string(),
        title: "delete".to_string(),
        side: SelectSide::Left,
        hidden: false,
        disabled: SelectDisabled::Flag(false),
        on_trigger: Box::new(|_| {}),
    }];
    state
}

/// Two-step delete arming (mirrors the `stash.delete` trigger up to the
/// store call; the caller removes when this returns true).
pub fn stash_delete_press(to_delete: &mut Option<usize>, index: usize) -> bool {
    if *to_delete == Some(index) {
        *to_delete = None;
        return true;
    }
    *to_delete = Some(index);
    false
}
