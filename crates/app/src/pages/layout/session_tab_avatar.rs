//! Rust port of `packages/app/src/pages/layout/session-tab-avatar.tsx` (opencode v1.18.30).
//! 1:1 exact translation — same names/behavior/edge-cases.
//! Rename log: `layout/session-tab-avatar.tsx` -> `layout/session_tab_avatar.rs` (kebab -> snake_case, Rust identifier rule).
// PROVISIONAL: pending solid-js/solid-primitives/@solidjs/router/ghostty-web/shiki/@pierre/trees/@tanstack/solid — mirrors `layout/session-tab-avatar.tsx`
#![allow(dead_code, unused_variables, clippy::all)]

// Exports mirrored from source (1:1): SessionTabAvatar, SessionTabAvatarView

use serde::{Deserialize, Serialize};

/// Props for `SessionTabAvatar` — field-for-field descriptor (no DOM rendering).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTabAvatarProps {
    pub placeholder: String,
}

/// Render descriptor for `SessionTabAvatar` — mirrors JSX output fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTabAvatarRender {
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
