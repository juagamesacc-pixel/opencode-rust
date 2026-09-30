//! Rust port of `packages/app/src/pages/session/timeline/measure.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/timeline/measure.ts` -> `session/timeline/measure.rs` (kebab -> snake_case).

// PROVISIONAL: DOM requestAnimationFrame + isConnected — mirrors scheduleConnectedMeasure
// Pending browser DOM — stub preserves signature
pub fn schedule_connected_measure(_connected: bool) -> Option<u32> {
    None
}
