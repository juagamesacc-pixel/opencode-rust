//! Rust port of `packages/core/src/process.ts`.

use serde::{Deserialize, Serialize};

pub const MAX_OUTPUT_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub exit_code: Option<i32>,
    pub output: Option<String>,
    pub truncated: bool,
}

pub fn is_timeout_message(msg: &str) -> bool {
    msg == "Timed out"
}

// PROVISIONAL pending AppProcess service wiring.
