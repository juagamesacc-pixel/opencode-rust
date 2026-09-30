//! Port of `packages/schema/src/worktree-event.ts`.
//!
//! Source exports: `Ready`, `Failed`, `Definitions` (`worktree.ready`
//! `{name, branch?}`, `worktree.failed` `{message}`). Uses the `Event.define`
//! namespace form in source.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Payload of `worktree.ready`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadyPayload {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

/// `worktree.ready` definition marker.
pub struct Ready;
impl Ready {
    pub const TYPE: &'static str = "worktree.ready";
}

/// Payload of `worktree.failed`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedPayload {
    pub message: String,
}

/// `worktree.failed` definition marker.
pub struct Failed;
impl Failed {
    pub const TYPE: &'static str = "worktree.failed";
}

/// Verbatim declaration order: `Ready`, `Failed`.
pub const Definitions: &[&'static str] = &[Ready::TYPE, Failed::TYPE];
