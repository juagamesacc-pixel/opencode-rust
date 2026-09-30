//! Port of `packages/schema/src/revert.ts`.
//!
//! Source exports: `FileDiff` (identifier `File.Diff`), `State` (identifier
//! `Revert.State`). Cross-lane: `NonNegativeInt`/`RelativePath` are
//! `crate::schema_primitives` (Lane A); `SessionMessage.ID` is
//! `crate::session_message::ID` (Lane B) per plan §4.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `File.Diff` `status` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FileDiffStatus {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "deleted")]
    Deleted,
}

/// `Revert.FileDiff`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileDiff {
    pub path: crate::schema_primitives::RelativePath,
    pub status: FileDiffStatus,
    pub additions: crate::schema_primitives::NonNegativeInt,
    pub deletions: crate::schema_primitives::NonNegativeInt,
    pub patch: String,
}

/// `Revert.State`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub messageID: crate::session_message::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partID: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileDiff>>,
}
