//! Rust port of `packages/app/src/pages/session/session-panel-layout.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/session-panel-layout.ts` -> `session/session_panel_layout.rs` (kebab -> snake_case).

pub struct SessionPanelLayoutInput {
    pub review: bool,
    pub terminal: bool,
    pub files: bool,
}
pub struct SessionPanelLayoutOutput {
    pub visible: bool,
    pub stacked: bool,
}
pub fn session_panel_layout(input: SessionPanelLayoutInput) -> SessionPanelLayoutOutput {
    SessionPanelLayoutOutput {
        visible: input.review || input.terminal || input.files,
        stacked: input.review && input.terminal,
    }
}
