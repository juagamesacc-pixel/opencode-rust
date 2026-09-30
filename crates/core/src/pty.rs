//! Rust port of `packages/core/src/pty.ts`.

use serde::{Deserialize, Serialize};

pub const BUFFER_LIMIT: usize = 1024 * 1024 * 2;
pub const EXITED_LIMIT: usize = 25;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub id: String,
    pub title: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub cols: u32,
    pub rows: u32,
}

#[derive(Debug, Clone)]
pub struct AttachInput {
    pub cursor: Option<i64>,
}

#[derive(Debug)]
pub struct NotFoundError {
    pub pty_id: String,
}
#[derive(Debug)]
pub struct ExitedError {
    pub pty_id: String,
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pty.NotFoundError: {}", self.pty_id)
    }
}
impl std::fmt::Display for ExitedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pty.ExitedError: {}", self.pty_id)
    }
}

// PROVISIONAL pending native pty binding (#pty) — state machine above is real, native spawn/onData/onExit pending crate.
