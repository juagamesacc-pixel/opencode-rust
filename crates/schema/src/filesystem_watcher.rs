//! Port of `packages/schema/src/filesystem-watcher.ts`.
//!
//! Source exports: `Event.{Updated,Definitions}` only (`file.watcher.updated`
//! with `file` + `event` closed set).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `file.watcher.updated` `event` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdatedEvent {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "change")]
    Change,
    #[serde(rename = "unlink")]
    Unlink,
}

/// Event definitions (`file.watcher.updated`).
pub mod Event {
    /// `file.watcher.updated` — payload [`UpdatedPayload`].
    pub struct Updated;
    impl Updated {
        pub const TYPE: &'static str = "file.watcher.updated";
    }

    /// Payload of `file.watcher.updated`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UpdatedPayload {
        pub file: String,
        pub event: super::UpdatedEvent,
    }

    /// Verbatim declaration order: `Updated`.
    pub const Definitions: &[&'static str] = &[Updated::TYPE];
}
