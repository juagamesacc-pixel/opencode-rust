//! Port of `packages/schema/src/workspace-id.ts`.
//!
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/error-
//! strings/keys/defaults/ordering. Source is spec.
//!
//! Source: `Schema.String` checked with `Schema.isStartsWith("wrk")` (no
//! underscore), branded `"WorkspaceV2.ID"`, with
//! `statics((schema) => ({ ascending, create }))` where
//! `create = () => schema.make("wrk_" + ascending())` and
//! `ascending = (id?: string) => { if (!id) return create(); if (!id.startsWith("wrk")) throw new Error(...); return schema.make(id) }`.
//! The throw message `` `ID ${id} does not start with wrk` `` is preserved
//! byte-identical (Rust: panics with that message).

#![allow(non_snake_case)]

use std::ops::Deref;

/// Branded workspace identifier (`WorkspaceID` in `workspace-id.ts`).
///
/// Wire format is a bare string. The `export const WorkspaceID` /
/// `export type WorkspaceID` pair shares one Rust item (structs are both).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct WorkspaceID(pub String);

impl WorkspaceID {
    /// Port of `create` (`() => schema.make("wrk_" + ascending())`).
    pub fn create() -> Self {
        WorkspaceID(format!("wrk_{}", crate::identifier::ascending()))
    }

    /// Port of `ascending` (`(id?: string) => ...`).
    ///
    /// `None` — or `Some("")`, mirroring source's falsy `!id` check — creates a
    /// fresh ID. A non-empty `id` that does not start with `"wrk"` throws
    /// `` `ID ${id} does not start with wrk` `` in source; Rust panics with the
    /// byte-identical message.
    pub fn ascending(id: Option<&str>) -> Self {
        match id {
            None => Self::create(),
            Some("") => Self::create(),
            Some(id) => {
                if !id.starts_with("wrk") {
                    panic!("ID {id} does not start with wrk");
                }
                WorkspaceID(id.to_string())
            }
        }
    }

    /// Port of the `Schema.isStartsWith("wrk")` check.
    pub fn is_valid(value: &str) -> bool {
        value.starts_with("wrk")
    }

    pub fn new(value: String) -> Self {
        WorkspaceID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for WorkspaceID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WorkspaceID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WorkspaceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for WorkspaceID {
    fn from(value: String) -> Self {
        WorkspaceID(value)
    }
}

impl From<&str> for WorkspaceID {
    fn from(value: &str) -> Self {
        WorkspaceID(value.to_string())
    }
}
