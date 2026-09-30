//! Rust port of `packages/core/src/session.ts`.

use serde::{Deserialize, Serialize};

pub const REVERT_STATE_KEY: &str = "revert";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NotFoundError {
    pub session_id: String,
}
#[derive(Debug, Clone)]
pub struct OperationUnavailableError {
    pub operation: String,
}
#[derive(Debug, Clone)]
pub struct PromptConflictError {
    pub session_id: String,
    pub message_id: String,
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session.NotFoundError: {}", self.session_id)
    }
}
impl std::fmt::Display for OperationUnavailableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session.OperationUnavailableError: {}", self.operation)
    }
}
impl std::fmt::Display for PromptConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Session.PromptConflictError: {} {}",
            self.session_id, self.message_id
        )
    }
}

pub fn resolve_prompt_mime(uri: &str, name: Option<&str>) -> String {
    if let Some(m) = uri
        .split(';')
        .next()
        .and_then(|s| s.split(':').nth(1))
        .map(|s| s.split(',').next().unwrap_or(s).to_string())
    {
        if !m.is_empty() {
            return m;
        }
    }
    let target = if uri.contains("://") {
        uri.split('/').next_back().unwrap_or(uri).to_string()
    } else {
        name.unwrap_or(uri).to_string()
    };
    if target.ends_with('/') {
        return "application/x-directory".to_string();
    }
    crate::filesystem::mime_type(&target)
}

pub mod compaction;
pub mod context_epoch;
pub mod error;
pub mod event;
pub mod execution;
pub mod history;
pub mod info;
pub mod input;
pub mod message;
pub mod message_updater;
pub mod projector;
pub mod prompt;
pub mod revert;
pub mod run_coordinator;
pub mod runner;
pub mod schema;
pub mod sql;
pub mod store;
pub mod todo;

// PROVISIONAL pending Database + EventV2 + SessionStore wiring — pure helpers above are verbatim.
