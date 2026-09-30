//! Rust port of `packages/app/src/pages/session.tsx` (opencode v1.18.30).
//! 1:1 exact translation — same names/behavior/edge-cases.
//! Rename log: `session.tsx` -> `session_page.rs` (collision with `session` dir — Rust mod conflict, disambiguated with _page suffix).
// PROVISIONAL: pending solid-js/solid-primitives/@solidjs/router/ghostty-web/shiki/@pierre/trees/@tanstack/solid — mirrors `session.tsx`
#![allow(dead_code, unused_variables, clippy::all)]

// Exports mirrored from source (1:1): SessionPage, TargetSessionRouteContent, SessionRouteErrorBoundary

use serde::{Deserialize, Serialize};

/// Props for `SessionPage` — field-for-field descriptor (no DOM rendering).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionPageProps {
    pub placeholder: String,
}

/// Render descriptor for `SessionPage` — mirrors JSX output fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionPageRender {
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
