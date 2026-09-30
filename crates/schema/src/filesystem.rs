//! Port of `packages/schema/src/filesystem.ts`.
//!
//! Source exports: `Event.{Edited,Definitions}`, `Entry`, `Submatch`, `Match`,
//! `FindInput` (source `Schema.Class`). Cross-lane: `NonNegativeInt`,
//! `PositiveInt`, `RelativePath` are `crate::schema_primitives` (Lane A).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Event definitions (`file.edited`).
pub mod Event {
    /// `file.edited` — payload [`EditedPayload`].
    pub struct Edited;
    impl Edited {
        pub const TYPE: &'static str = "file.edited";
    }

    /// Payload of `file.edited`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditedPayload {
        pub file: String,
    }

    /// Verbatim declaration order: `Edited`.
    pub const Definitions: &[&'static str] = &[Edited::TYPE];
}

/// `FileSystem.Entry` `type` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntryType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory,
}

/// `FileSystem.Entry`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub path: crate::schema_primitives::RelativePath,
    #[serde(rename = "type")]
    pub r#type: EntryType,
}

/// `FileSystem.Submatch`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Submatch {
    pub text: String,
    pub start: crate::schema_primitives::NonNegativeInt,
    pub end: crate::schema_primitives::NonNegativeInt,
}

/// `FileSystem.Match`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub entry: Entry,
    pub line: crate::schema_primitives::PositiveInt,
    pub offset: crate::schema_primitives::NonNegativeInt,
    pub text: String,
    pub submatches: Vec<Submatch>,
}

/// `FileSystem.FindInput` `type` filter closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FindInputType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory,
}

/// `FileSystem.FindInput` (source `Schema.Class`; `query` required,
/// constructed positionally as `new FindInput({ query })` in source).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FindInput {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FindInputType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::schema_primitives::PositiveInt>,
}
