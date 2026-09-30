//! Rust port of `packages/app/src/context/file/types.ts` (opencode v1.18.30).
//!
//! Source 42 lines. Exports: `FileSelection`, `SelectedLineRange`, `FileViewState`, `FileState`, `selectionFromLines`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/file/types.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `FileSelection`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSelection {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `SelectedLineRange`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedLineRange {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `FileViewState`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileViewState {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `FileState`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileState {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `selectionFromLines`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/types.ts
#[allow(non_snake_case)]
pub fn selectionFromLines(/* range: SelectedLineRange */) -> serde_json::Value {
    serde_json::json!({})
}
