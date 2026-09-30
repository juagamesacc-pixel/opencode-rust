//! Rust port of `packages/app/src/pages/session/message-gesture.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/message-gesture.ts` -> `session/message_gesture.rs` (kebab -> snake_case).

pub struct WheelInput {
    pub delta_y: f64,
    pub delta_mode: i32,
    pub root_height: f64,
}
pub fn normalize_wheel_delta(input: WheelInput) -> f64 {
    if input.delta_mode == 1 {
        return input.delta_y * 40.0;
    }
    if input.delta_mode == 2 {
        return input.delta_y * input.root_height;
    }
    input.delta_y
}

pub struct BoundaryInput {
    pub delta: f64,
    pub scroll_top: f64,
    pub scroll_height: f64,
    pub client_height: f64,
}
pub fn should_mark_boundary_gesture(input: BoundaryInput) -> bool {
    let max = input.scroll_height - input.client_height;
    if max <= 1.0 {
        return true;
    }
    if input.delta == 0.0 {
        return false;
    }
    if input.delta < 0.0 {
        return input.scroll_top + input.delta <= 0.0;
    }
    let remaining = max - input.scroll_top;
    input.delta > remaining
}
