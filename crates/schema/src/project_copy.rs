//! Rust port of `packages/schema/src/project-copy.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! `StrategyID` is `Schema.Trim.pipe(Schema.check(Schema.isNonEmpty()), Schema.brand("ProjectCopy.StrategyID"))`
//! — a branded string; the non-empty check is exposed as `is_valid` (runtime),
//! the wire format stays a bare string.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Brand `ProjectCopy.StrategyID` — transparent newtype, wire format stays bare string.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StrategyID(pub String);

impl StrategyID {
    /// Port of the `Schema.isNonEmpty()` check after `Schema.Trim` (which trims
    /// whitespace first). Source: `Schema.check(isNonEmpty)` on a trimmed string.
    pub fn is_valid(value: &str) -> bool {
        !value.trim().is_empty()
    }

    pub fn new(value: String) -> Self {
        StrategyID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for StrategyID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for StrategyID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StrategyID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for StrategyID {
    fn from(value: String) -> Self {
        StrategyID(value)
    }
}

impl From<&str> for StrategyID {
    fn from(value: &str) -> Self {
        StrategyID(value.to_string())
    }
}

/// Identifier `ProjectCopy.CreateInput`. Field order is verbatim:
/// projectID, strategy, sourceDirectory, directory, name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInput {
    pub projectID: crate::project_id::ProjectID,
    pub strategy: StrategyID,
    pub sourceDirectory: crate::schema_primitives::AbsolutePath,
    pub directory: crate::schema_primitives::AbsolutePath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Identifier `ProjectCopy.RemoveInput`. Field order is verbatim:
/// projectID, directory, force.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveInput {
    pub projectID: crate::project_id::ProjectID,
    pub directory: crate::schema_primitives::AbsolutePath,
    pub force: bool,
}

/// Identifier `ProjectCopy.Copy`. Field order is verbatim: directory.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Copy {
    pub directory: crate::schema_primitives::AbsolutePath,
}
