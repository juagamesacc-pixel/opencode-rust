//! Rust port of `packages/schema/src/location.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! Source exports `Ref` (struct), `Info` (`Schema.Class`) and `response(data)`
//! (a schema builder returning the anonymous struct `{ location: Info, data }`).
//! The anonymous `project` sub-struct is ported as [`InfoProject`]; the
//! anonymous return shape of `response(...)` is ported as the generic struct
//! [`Response`] (there is no codec-object system to port the builder fn, and a
//! generic struct is the only way to carry the `data` slot in Rust).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Identifier `Location.Ref`. Field order is verbatim:
/// directory, workspaceID.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ref {
    pub directory: crate::schema_primitives::AbsolutePath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaceID: Option<crate::workspace_id::WorkspaceID>,
}

/// Identifier `Location.Info` (`Schema.Class<Info>("Location.Info")`).
///
/// Field order is verbatim: directory, workspaceID, project.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    pub directory: crate::schema_primitives::AbsolutePath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaceID: Option<crate::workspace_id::WorkspaceID>,
    pub project: InfoProject,
}

/// Anonymous `project` sub-struct of `Info`
/// (`Schema.Struct({ id: ProjectID, directory: AbsolutePath })`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProject {
    pub id: crate::project_id::ProjectID,
    pub directory: crate::schema_primitives::AbsolutePath,
}

/// Port of the return shape of `response(data)` (`Schema.Struct({ location: Info, data })`).
///
/// The source function is a schema builder (there is no Rust codec object);
/// this generic struct carries the exact wire shape with `data` of any type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response<Data> {
    pub location: Info,
    pub data: Data,
}
