//! Rust port of `packages/app/src/pages/session/helpers.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/helpers.ts` -> `session/helpers.rs` (kebab -> snake_case).

pub fn get_session_key(dir: Option<&str>, id: Option<&str>) -> String {
    let d = dir.unwrap_or("");
    match id {
        Some(v) if !v.is_empty() => format!("{}/{}", d, v),
        _ => d.to_string(),
    }
}

pub fn should_show_file_tree(visible: bool, opened: bool) -> bool {
    opened && visible
}

pub fn get_tab_reorder_index(tabs: &[String], from: &str, to: &str) -> Option<usize> {
    let from_idx = tabs.iter().position(|x| x == from)?;
    let to_idx = tabs.iter().position(|x| x == to)?;
    if from_idx == to_idx {
        return None;
    }
    Some(to_idx)
}

// PROVISIONAL: solid-js createSessionTabs / focusTerminalById / createOpenReviewFile — browser/solid only
