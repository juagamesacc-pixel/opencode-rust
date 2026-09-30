//! Rust port of `src/renderer/window-fullscreen.ts` (opencode v1.18.30).
//!
//! The `createSignal(false)` default is ported; the `window.api`
//! subscription (`onWindowFullscreenChanged`) and initial fetch
//! (`getWindowFullscreen().then(…)`) are PROVISIONAL (Solid + bridge
//! runtime). Only `windowFullscreen` is exported in the source.
//!
//! Original file: `packages/desktop/src/renderer/window-fullscreen.ts`

/// Mirrors the `windowFullscreen` signal value (`createSignal(false)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WindowFullscreenState {
    pub fullscreen: bool,
}

impl WindowFullscreenState {
    pub fn new() -> Self {
        Self { fullscreen: false }
    }

    /// Mirrors both `setWindowFullscreen` call sites (the fullscreen-changed
    /// subscription and the initial `getWindowFullscreen()` fetch).
    pub fn set(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }
}

// PROVISIONAL(packages/desktop/src/renderer/window-fullscreen.ts):
// `window.api.onWindowFullscreenChanged(setWindowFullscreen)` and
// `window.api.getWindowFullscreen().then(setWindowFullscreen)` need the
// Solid signal + `window.api` bridge.
pub fn subscribe_window_fullscreen(_state: &mut WindowFullscreenState) {
    unimplemented!("Solid signal + window.api bridge binding")
}
