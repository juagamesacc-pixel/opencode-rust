//! Rust port of `packages/core/src/ripgrep.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepInput {
    pub cwd: String,
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobInput {
    pub cwd: String,
    pub pattern: String,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEntry {
    pub path: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub entry: MatchEntry,
    pub line: u64,
    pub text: String,
}

// PROVISIONAL pending native ripgrep binding — interface above is verbatim, execution requires ripgrep crate.
