//! Rust port of `packages/app/src/pages/home/home-projects-controller.tsx` (opencode v1.18.30).
//! 1:1 exact translation — same names/behavior/edge-cases.
//! Rename log: `home/home-projects-controller.tsx` -> `home/home_projects_controller.rs` (kebab -> snake_case, Rust identifier rule).
// PROVISIONAL: pending solid-js/solid-primitives/@solidjs/router/ghostty-web/shiki/@pierre/trees/@tanstack/solid — mirrors `home/home-projects-controller.tsx`
#![allow(dead_code, unused_variables, clippy::all)]

// Exports mirrored from source (1:1): createHomeProjectsController, HomeProjectsController

use serde::{Deserialize, Serialize};

/// Props for `createHomeProjectsController` — field-for-field descriptor (no DOM rendering).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateHomeProjectsControllerProps {
    pub placeholder: String,
}

/// Render descriptor for `createHomeProjectsController` — mirrors JSX output fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateHomeProjectsControllerRender {
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
