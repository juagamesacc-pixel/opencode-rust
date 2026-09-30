// source: packages/tui/src/component/register-spinner.ts (6 lines, v1.18.30)
// 1:1 port — the component-catalogue registration is a no-op marker
// (ratatui has no catalogue); frames live in `ui::spinner`.

#![allow(dead_code)]

/// Mirrors `registerOpencodeSpinner` — records that the braille spinner
/// frames from `ui::spinner` are the canonical spinner.
pub fn register_opencode_spinner() {}
