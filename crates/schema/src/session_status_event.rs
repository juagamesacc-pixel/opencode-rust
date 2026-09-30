//! 1:1 port of packages/schema/src/session-status-event.ts
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryAction {
    pub reason: String,
    pub provider: String,
    pub title: String,
    pub message: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoIdle {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoRetry {
    pub attempt: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RetryAction>,
    pub next: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoBusy {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Info {
    #[serde(rename = "idle")]
    Idle(InfoIdle),
    #[serde(rename = "retry")]
    Retry(InfoRetry),
    #[serde(rename = "busy")]
    Busy(InfoBusy),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub sessionID: crate::session_id::SessionID,
    pub status: Info,
}
impl Status {
    pub const TYPE: &'static str = "session.status";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Idle {
    pub sessionID: crate::session_id::SessionID,
}
impl Idle {
    pub const TYPE: &'static str = "session.idle";
}
pub mod Event {
    pub use super::{Idle, Status};
    pub const Definitions: &[&'static str] = &[Status::TYPE, Idle::TYPE];
}
