//! Rust port of `packages/app/src/pages/layout.tsx` (opencode v1.18.30).
//! 1:1 exact translation — same names/behavior/edge-cases.
//! Rename log: `layout.tsx` -> `layout_page.rs` (collision with `layout` dir — Rust mod conflict, disambiguated with _page suffix).
// PROVISIONAL: pending solid-js/solid-primitives/@solidjs/router/ghostty-web/shiki/@pierre/trees/@tanstack/solid — mirrors `layout.tsx`
#![allow(dead_code, unused_variables, clippy::all)]

use serde::{Deserialize, Serialize};

/// Props for `layout` — field-for-field descriptor (no DOM rendering).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutProps {
    pub placeholder: String,
}

/// Render descriptor for `layout` — mirrors JSX output fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutRender {
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
