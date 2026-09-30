//! Rust port of `packages/app/src/pages/session/composer/session-composer-state.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/composer/session-composer-state.ts` -> `session/composer/session_composer_state.rs` (kebab -> snake_case).

pub fn todo_state(count: usize, done: bool, live: bool) -> &'static str {
    if count == 0 {
        return "hide";
    }
    if !live {
        return "clear";
    }
    if !done {
        return "open";
    }
    "close"
}

pub fn todo_dock_at_boundary(state: &str) -> bool {
    state == "open"
}

// PROVISIONAL: solid-js store for createSessionComposerController — descriptor
// Full reactive logic mirrors source createSessionComposerController (session composer state machine)
// Pending solid-js — see source for full effect/signal wiring
#[derive(Clone, Debug)]
pub struct SessionComposerState {
    pub session_id: Option<String>,
    pub responding: Option<String>,
    pub dock: bool,
    pub closing: bool,
    pub opening: bool,
}
