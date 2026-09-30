//! Rust port of `packages/core/src/tool/tool.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationError {
    pub name: String,
    pub message: String,
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

impl std::error::Error for RegistrationError {}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolContent {
    Text {
        text: String,
    },
    File {
        data: String,
        mime: String,
        name: Option<String>,
    },
}

pub fn validate_name(name: &str) -> Result<(), RegistrationError> {
    let valid = name.len() <= 64
        && !name.is_empty()
        && name
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic())
            .unwrap_or(false)
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if valid {
        Ok(())
    } else {
        Err(RegistrationError {
            name: name.to_string(),
            message: format!("Invalid tool name: {name}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub session_id: String,
    pub agent: String,
    pub assistant_message_id: String,
    pub tool_call_id: String,
}

// Definition mirrors opaque Tool.make — runtime stored via registry, not here.
// PROVISIONAL pending Effect schema — serde validation preserves wire names.
// Real behavior: validate_name, permission defaulting, definition derivation via registry.

pub fn permission_for_tool(permission: Option<&str>, name: &str) -> String {
    permission.unwrap_or(name).to_string()
}

// PROVISIONAL pending WeakMap runtime + settle — requires LLM ToolCall types.
