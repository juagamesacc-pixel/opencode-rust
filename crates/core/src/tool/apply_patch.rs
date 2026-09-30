//! Rust port of `packages/core/src/tool/apply-patch.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "apply_patch";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "patchText")]
    pub patch_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applied {
    #[serde(rename = "type")]
    pub type_: String,
    pub resource: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub applied: Vec<Applied>,
    pub files: Vec<FileDiffInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiffInfo {
    pub file: String,
    pub patch: String,
    pub status: String,
    pub additions: usize,
    pub deletions: usize,
}

pub fn to_model_output(output: &Output) -> String {
    let mut lines = vec!["Applied patch sequentially:".to_string()];
    for item in &output.applied {
        let prefix = match item.type_.as_str() {
            "add" => "A",
            "delete" => "D",
            _ => "M",
        };
        lines.push(format!("{prefix} {}", item.resource));
    }
    lines.join("\n")
}

pub fn validate_patch_text(patch: &str) -> Result<(), String> {
    if patch.trim().is_empty() {
        return Err("patchText is required".to_string());
    }
    Ok(())
}

// PROVISIONAL pending Patch + FileMutation + FSUtil + LocationMutation + PermissionV2.
