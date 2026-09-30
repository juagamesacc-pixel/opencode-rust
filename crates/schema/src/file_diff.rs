//! Port of `packages/schema/src/file-diff.ts`.
//!
//! Source exports: `Info` only (identifier `SnapshotFileDiff`). `Finite` maps
//! to `f64`; package `optional()` maps to `Option` + omit-when-absent.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `SnapshotFileDiff` `status` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "modified")]
    Modified,
}

/// `FileDiff.Info`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
    pub additions: f64,
    pub deletions: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}
