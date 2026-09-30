//! Rust port of `packages/core/src/skill.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub location: String,
    pub content: String,
}

pub fn skill_dir(location: &str) -> String {
    std::path::Path::new(location)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

// PROVISIONAL pending Skill discovery filesystem wiring.
