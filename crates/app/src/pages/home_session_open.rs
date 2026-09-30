//! Rust port of `packages/app/src/pages/home-session-open.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `home-session-open.ts` -> `home_session_open.rs` (kebab -> snake_case).

pub struct ShouldOpenInput {
    pub button: i32,
    pub mac: bool,
    pub meta: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

pub fn should_open_session_in_background(input: ShouldOpenInput) -> bool {
    if input.button == 1 {
        return true;
    }
    if input.button != 0 {
        return false;
    }
    if input.shift || input.alt {
        return false;
    }
    if input.mac {
        return input.meta && !input.ctrl;
    }
    input.ctrl && !input.meta
}
