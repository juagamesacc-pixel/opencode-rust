//! Port of `packages/schema/src/prompt.ts`.
//!
//! Source exports: `Source`, `FileAttachment` (with `create`), `AgentAttachment`,
//! `Prompt` (with `equivalence` + `fromUserMessage`). Self-contained: `Finite`
//! maps to `f64`, package `optional()` maps to `Option` + omit-when-absent.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `Prompt.Source`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

/// `Prompt.FileAttachment`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileAttachment {
    pub uri: String,
    pub mime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl FileAttachment {
    /// Canonical constructor (copies input fields, preserving absent keys).
    pub fn create(input: FileAttachment) -> Self {
        Self {
            uri: input.uri,
            mime: input.mime,
            name: input.name,
            description: input.description,
            source: input.source,
        }
    }
}

/// `Prompt.AgentAttachment`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentAttachment {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

/// `Prompt`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<AgentAttachment>>,
}

impl Prompt {
    /// Structural equality (ports `Schema.toEquivalence`).
    pub fn equivalence(a: &Prompt, b: &Prompt) -> bool {
        a == b
    }

    /// Constructor omitting `undefined` (`None`) keys, as in source.
    pub fn fromUserMessage(input: Prompt) -> Self {
        Self {
            text: input.text,
            files: input.files,
            agents: input.agents,
        }
    }
}
