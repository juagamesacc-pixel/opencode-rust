//! Rust port of `packages/core/src/command.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
}

pub fn validate_command_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Command name required".to_string());
    }
    Ok(())
}

// PROVISIONAL pending command registry wiring.
