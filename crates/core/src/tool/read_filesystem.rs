//! Rust port of `packages/core/src/tool/read-filesystem.ts`.

use serde::{Deserialize, Serialize};

pub const DEFAULT_PAGE_LIMIT: u64 = 2000;
pub const MAX_LINE_LENGTH: usize = 2000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPage {
    pub content: String,
    pub total_lines: u64,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPage {
    pub entries: Vec<String>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug)]
pub struct BinaryFileError {
    pub resource: String,
}
#[derive(Debug)]
pub struct MediaIngestLimitError {
    pub resource: String,
}

impl std::fmt::Display for BinaryFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binary file: {}", self.resource)
    }
}
impl std::fmt::Display for MediaIngestLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Media ingest limit exceeded: {}", self.resource)
    }
}

// PROVISIONAL pending FSUtil + Image wiring.
