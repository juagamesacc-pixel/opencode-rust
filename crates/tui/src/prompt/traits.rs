// source: packages/tui/src/prompt/traits.ts (29 lines, v1.18.30)
// 1:1 port — prompt textarea traits verbatim (capture sets + SHELL status).

#![allow(dead_code)]

/// `PromptMode` lives in `history` (mirrors the shared mode union).
pub use super::history::PromptMode;

/// Capture sets verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptCapture {
    EscapeNavigateSubmitTab,
    TabOnly,
    None,
}

/// Mirrors `PromptTraits` (owner/role constant).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromptTraits {
    pub capture: PromptCapture,
    pub status_shell: bool,
    pub owner_is_opencode: bool,
    pub role_is_prompt: bool,
}

/// Mirrors `computePromptTraits`.
pub fn compute_prompt_traits(mode: PromptMode, autocomplete_visible: bool) -> PromptTraits {
    let capture = match mode {
        PromptMode::Normal if autocomplete_visible => PromptCapture::EscapeNavigateSubmitTab,
        PromptMode::Normal => PromptCapture::TabOnly,
        PromptMode::Shell => PromptCapture::None,
    };
    PromptTraits {
        capture,
        status_shell: mode == PromptMode::Shell,
        owner_is_opencode: true,
        role_is_prompt: true,
    }
}

/// Capture key names for the keymap layer.
pub fn capture_keys(capture: PromptCapture) -> &'static [&'static str] {
    match capture {
        PromptCapture::EscapeNavigateSubmitTab => &["escape", "navigate", "submit", "tab"],
        PromptCapture::TabOnly => &["tab"],
        PromptCapture::None => &[],
    }
}
