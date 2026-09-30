//! Port of `packages/schema/src/project-id.ts`.
//!
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/error-
//! strings/keys/defaults/ordering. Source is spec.
//!
//! Source: `Schema.String` branded `"Project.ID"` with
//! `statics((schema) => ({ global: schema.make("global") }))`. No refinement
//! check in source — any string is accepted (no validation added here).

#![allow(non_snake_case)]

use std::ops::Deref;

/// Branded project identifier (`ProjectID` in `project-id.ts`).
///
/// Wire format is a bare string. The `export const ProjectID` /
/// `export type ProjectID` pair shares one Rust item (structs are both).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ProjectID(pub String);

impl ProjectID {
    /// Port of `global` (`schema.make("global")`).
    pub fn global() -> Self {
        ProjectID("global".to_string())
    }

    pub fn new(value: String) -> Self {
        ProjectID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for ProjectID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ProjectID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ProjectID {
    fn from(value: String) -> Self {
        ProjectID(value)
    }
}

impl From<&str> for ProjectID {
    fn from(value: &str) -> Self {
        ProjectID(value.to_string())
    }
}
