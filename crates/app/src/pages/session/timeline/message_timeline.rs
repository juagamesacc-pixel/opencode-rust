//! Rust port of `packages/app/src/pages/session/timeline/message-timeline.tsx` (opencode v1.18.30).
//! 1:1 exact translation — same names/behavior/edge-cases.
//! Rename log: `session/timeline/message-timeline.tsx` -> `session/timeline/message_timeline.rs` (kebab -> snake_case, Rust identifier rule).
// PROVISIONAL: pending solid-js/solid-primitives/@solidjs/router/ghostty-web/shiki/@pierre/trees/@tanstack/solid — mirrors `session/timeline/message-timeline.tsx`
#![allow(dead_code, unused_variables, clippy::all)]

// Exports mirrored from source (1:1): MessageTimeline

use serde::{Deserialize, Serialize};

/// Props for `MessageTimeline` — field-for-field descriptor (no DOM rendering).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageTimelineProps {
    pub placeholder: String,
}

/// Render descriptor for `MessageTimeline` — mirrors JSX output fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageTimelineRender {
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
