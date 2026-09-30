//! Port of `packages/schema/src/permission-saved.ts`.
//!
//! Source exports: `ID` (brand `PermissionSaved.ID`, no prefix check) and
//! `Info`. Cross-lane: `ProjectID` is `crate::project_id::ProjectID` (Lane A).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Branded `PermissionSaved.ID` (no prefix check in source).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Canonical constructor (`psv_` + ascending identifier).
    pub fn create() -> Self {
        Self(format!("psv_{}", crate::identifier::ascending()))
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// `PermissionSaved.Info`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    pub projectID: crate::project_id::ProjectID,
    pub action: String,
    pub resource: String,
}
