//! Port of `packages/schema/src/prompt-input.ts`.
//!
//! Source exports: `FileAttachment` (with `create`, note: no `mime` field unlike
//! `prompt.ts`) and `Prompt`. Kept as a separate module per doctrine (no merge
//! with `prompt.rs`); `AgentAttachment`/`Source` are referenced from there.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `PromptInput.FileAttachment`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileAttachment {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::prompt::Source>,
}

impl FileAttachment {
    /// Canonical constructor.
    pub fn create(input: FileAttachment) -> Self {
        input
    }
}

/// `PromptInput` (`Prompt`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<crate::prompt::AgentAttachment>>,
}
