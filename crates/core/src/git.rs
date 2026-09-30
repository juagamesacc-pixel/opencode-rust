//! Rust port of `packages/core/src/git.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub branch: String,
    pub clean: bool,
}

pub fn is_git_repo(path: &str) -> bool {
    std::path::Path::new(path).join(".git").exists()
}

pub fn branch_name_fallback() -> String {
    "main".to_string()
}

// PROVISIONAL pending git process spawning — pure helpers above are real.
