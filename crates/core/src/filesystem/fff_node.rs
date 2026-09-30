//! Rust port of `packages/core/src/filesystem/fff.node.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Result<T> {
    Ok { ok: bool, value: T },
    Err { ok: bool, error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    pub base_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frecency_db_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_db_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub relative_path: String,
    pub file_name: String,
    pub modified: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub relative_path: String,
    pub dir_name: String,
    pub max_access_frecency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Mixed {
    #[serde(rename = "file")]
    File { item: File },
    #[serde(rename = "directory")]
    Directory { item: Directory },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search {
    pub items: Vec<File>,
    pub scores: Vec<serde_json::Value>,
    pub total_matched: usize,
    pub total_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirSearch {
    pub items: Vec<Directory>,
    pub scores: Vec<serde_json::Value>,
    pub total_matched: usize,
    pub total_dirs: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedSearch {
    pub items: Vec<Mixed>,
    pub scores: Vec<serde_json::Value>,
    pub total_matched: usize,
    pub total_files: usize,
    pub total_dirs: usize,
}

pub type Cursor = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub relative_path: String,
    pub file_name: String,
    pub line_number: usize,
    pub byte_offset: usize,
}
