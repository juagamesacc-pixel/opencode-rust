//! Rust port of `packages/app/src/pages/session/session-panel-width.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/session-panel-width.ts` -> `session/session_panel_width.rs` (kebab -> snake_case).

pub const SESSION_PANEL_WIDTH_MIN: i32 = 450;
pub const REVIEW_PANE_WIDTH_MIN: i32 = 480;
pub const REVIEW_PANE_WIDTH_MIN_SPLIT: i32 = 800;

pub fn session_panel_width_max(available: i32, split: bool) -> i32 {
    let pane = if split {
        REVIEW_PANE_WIDTH_MIN_SPLIT
    } else {
        REVIEW_PANE_WIDTH_MIN
    };
    std::cmp::max(SESSION_PANEL_WIDTH_MIN, available - pane)
}

pub fn clamp_session_panel_width(width: i32, available: Option<i32>, split: bool) -> i32 {
    match available {
        None => width,
        Some(a) => std::cmp::min(width, session_panel_width_max(a, split)),
    }
}
