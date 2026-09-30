//! Rust port of `packages/core/src/tool/todowrite.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "todowrite";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoInfo {
    pub content: String,
    pub status: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub todos: Vec<TodoInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub todos: Vec<TodoInfo>,
}

pub fn to_model_output(output: &Output) -> String {
    serde_json::to_string_pretty(&output.todos).unwrap_or_else(|_| "[]".to_string())
}

// PROVISIONAL pending SessionTodo + PermissionV2 wiring.
