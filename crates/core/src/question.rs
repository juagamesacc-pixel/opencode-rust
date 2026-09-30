//! Rust port of `packages/core/src/question.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub question: String,
    pub header: String,
    pub options: Vec<PromptOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptOption {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub type Answer = Vec<String>;

// PROVISIONAL pending Question service wiring.
