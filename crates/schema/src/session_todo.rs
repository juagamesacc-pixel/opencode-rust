//! 1:1 port of packages/schema/src/session-todo.ts
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub content: String,
    pub status: String,
    pub priority: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Updated {
    pub sessionID: crate::session_id::SessionID,
    pub todos: Vec<Info>,
}
impl Updated {
    pub const TYPE: &'static str = "todo.updated";
}
pub mod Event {
    pub use super::Updated;
    pub const Definitions: &[&'static str] = &[Updated::TYPE];
}
