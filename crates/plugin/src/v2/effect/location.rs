// source: packages/plugin/src/v2/effect/location.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/location.ts` (opencode v1.18.30).
//!
//! Source 6 lines. Exports: `Location` with directory, project.directory.

use serde::{Deserialize, Serialize};

/// Mirrors `Location` verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub directory: String,
    pub project: LocationProject,
}

/// Mirrors `Location.project`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationProject {
    pub directory: String,
}
