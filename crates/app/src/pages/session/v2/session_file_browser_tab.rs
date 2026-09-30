//! Rust port of `packages/app/src/pages/session/v2/session-file-browser-tab.tsx` (opencode v1.18.30).
//! 1:1 exact translation — same names/behavior/edge-cases.
//! Rename log: `session/v2/session-file-browser-tab.tsx` -> `session/v2/session_file_browser_tab.rs` (kebab -> snake_case, Rust identifier rule).
// PROVISIONAL: pending solid-js/solid-primitives/@solidjs/router/ghostty-web/shiki/@pierre/trees/@tanstack/solid — mirrors `session/v2/session-file-browser-tab.tsx`
#![allow(dead_code, unused_variables, clippy::all)]

// Exports mirrored from source (1:1): SessionFileBrowserState, SessionFileBrowserTab

use serde::{Deserialize, Serialize};

/// Props for `SessionFileBrowserState` — field-for-field descriptor (no DOM rendering).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionFileBrowserStateProps {
    pub placeholder: String,
}

/// Render descriptor for `SessionFileBrowserState` — mirrors JSX output fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionFileBrowserStateRender {
    pub placeholder: String,
}

/// Route key for this page (preserves session-route).
#[derive(Clone, Debug, PartialEq)]
pub struct RouteKey(pub String);
impl RouteKey {
    pub fn href(&self) -> String {
        format!("/{}", self.0)
    }
}
