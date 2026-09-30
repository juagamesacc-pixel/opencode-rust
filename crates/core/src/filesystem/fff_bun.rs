//! Rust port of `packages/core/src/filesystem/fff.bun.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

/// Source: `export type Result<T> = { ok: true; value: T } | { ok: false; error: string }` verbatim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Result<T> {
    Ok { ok: bool, value: T },
    Err { ok: bool, error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search {
    pub items: Vec<serde_json::Value>,
    pub scores: serde_json::Value,
    pub total_matched: usize,
    pub total_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirSearch {
    pub items: Vec<serde_json::Value>,
    pub scores: serde_json::Value,
    pub total_matched: usize,
    pub total_dirs: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedSearch {
    pub items: Vec<serde_json::Value>,
    pub scores: serde_json::Value,
    pub total_matched: usize,
    pub total_files: usize,
    pub total_dirs: usize,
}

pub type File = serde_json::Value;
pub type Directory = serde_json::Value;
pub type Mixed = serde_json::Value;
pub type Cursor = Option<serde_json::Value>;
pub type Hit = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grep {
    pub items: Vec<serde_json::Value>,
    pub total_matched: usize,
    pub total_files_searched: usize,
    pub total_files: usize,
    pub filtered_file_count: usize,
    pub next_cursor: Cursor,
}

// PROVISIONAL pending @ff-labs/fff-bun FileFinder native binding
pub const BINDING: &str = "@ff-labs/fff-bun";
