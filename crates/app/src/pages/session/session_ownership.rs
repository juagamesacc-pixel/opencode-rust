//! Rust port of `packages/app/src/pages/session/session-ownership.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/session-ownership.ts` -> `session/session_ownership.rs` (kebab -> snake_case).

// PROVISIONAL: solid-js createComputed/onCleanup — mirrors createSessionOwnership
#[derive(Clone, Debug)]
pub struct SessionOwnership {
    pub generation: u64,
    pub current: String,
}
impl SessionOwnership {
    pub fn new(current: String) -> Self {
        Self {
            generation: 0,
            current,
        }
    }
    pub fn key(&self) -> String {
        format!("{}:{}", self.generation, self.current)
    }
}
