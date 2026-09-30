//! Rust port of `packages/core/src/tool/glob.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "glob";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub path: String,
}

pub fn to_model_output(output: &[Entry]) -> String {
    if output.is_empty() {
        "No files found".to_string()
    } else {
        output
            .iter()
            .map(|e| e.path.clone())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// PROVISIONAL pending effect/runtime — Layer wiring requires Ripgrep + Location + PermissionV2.
