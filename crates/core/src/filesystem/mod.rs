//! Rust port of `packages/core/src/filesystem` barrel + `packages/core/src/filesystem.ts` root items.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! (Merged: filesystem.rs folded here — E0761 single-module rule; items byte-identical, only relocated.)

pub mod fff_bun;
pub mod fff_node;
pub mod ignore;
pub mod protected;
pub mod search;
pub mod watcher;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadInput {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub content: String,
    pub encoding: String,
    pub mime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobInput {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepInput {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub path: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub entry: Entry,
    pub line: u64,
    pub text: String,
}

pub fn mime_type(path: &str) -> String {
    // Mirrors FSUtil.mimeType — extension based
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "webp" => "image/webp".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "json" => "application/json".to_string(),
        "html" | "htm" => "text/html".to_string(),
        "txt" | "md" => "text/plain".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

pub fn contains(root: &str, candidate: &str) -> bool {
    let root = std::path::Path::new(root);
    let cand = std::path::Path::new(candidate);
    cand.starts_with(root)
}

// PROVISIONAL pending FSUtil + FileSystemSearch service wiring.
