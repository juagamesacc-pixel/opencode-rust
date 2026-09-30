//! Rust port of `packages/app/src/pages/session/terminal-label.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/terminal-label.ts` -> `session/terminal_label.rs` (kebab -> snake_case).

pub fn terminal_tab_label(
    title: Option<&str>,
    title_number: Option<i32>,
    t: impl Fn(&str) -> String,
) -> String {
    let title = title.unwrap_or("");
    let number = title_number.unwrap_or(0);
    // isDefaultTitle check simplified — mirrors isDefaultTerminalTitle
    let is_default = number > 0 && title.contains(&number.to_string());
    if !title.is_empty() && !is_default {
        return title.to_string();
    }
    if number > 0 {
        return t("terminal.title.numbered");
    }
    if !title.is_empty() {
        return title.to_string();
    }
    t("terminal.title")
}
