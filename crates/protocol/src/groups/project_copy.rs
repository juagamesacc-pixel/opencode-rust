//! Rust port of `packages/protocol/src/groups/project-copy.ts`
//! (opencode v1.18.30).
//!
//! 1:1 exact translation — `ProjectCopyError` (stays in this module, NOT
//! merged into `crate::errors`), the create/remove payload shapes
//! (`ProjectCopy.CreateInput` minus `projectID`/`sourceDirectory`,
//! `ProjectCopy.RemoveInput` minus `projectID`), and the
//! `projectCopy.create` / `projectCopy.remove` / `projectCopy.refresh`
//! endpoints on `/experimental/project/:projectID/copy`
//! (`httpApiStatus: 400`).

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// The group root path verbatim from source
/// (`const root = "/experimental/project/:projectID/copy"`).
pub const PROJECT_COPY_ROOT: &str = "/experimental/project/:projectID/copy";

/// The refresh path verbatim from source (`` `${root}/refresh` ``).
pub const PROJECT_COPY_REFRESH_PATH: &str = "/experimental/project/:projectID/copy/refresh";

/// `ProjectCopyError` tag verbatim from source (`name: "ProjectCopyError"`).
pub const PROJECT_COPY_ERROR_TAG: &str = "ProjectCopyError";

/// `ProjectCopyError` status verbatim from source (`httpApiStatus: 400`).
pub const PROJECT_COPY_ERROR_HTTP_STATUS: u16 = 400;

/// Port of the `ProjectCopyError` `data` struct
/// (`{ message: string, forceRequired (optional) }`).
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectCopyErrorData {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forceRequired: Option<bool>,
}

/// Port of `ProjectCopyError` (`Schema.ErrorClass("ProjectCopyError")`).
///
/// Stays in this module per the nonsplit rule; it is not merged into
/// `crate::errors`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectCopyError {
    pub name: String,
    pub data: ProjectCopyErrorData,
}

impl ProjectCopyError {
    /// Create an error with the verbatim `name` (`"ProjectCopyError"`).
    pub fn new(message: String, force_required: Option<bool>) -> Self {
        ProjectCopyError {
            name: PROJECT_COPY_ERROR_TAG.to_string(),
            data: ProjectCopyErrorData {
                message,
                forceRequired: force_required,
            },
        }
    }

    /// Verbatim `httpApiStatus` code (`400`).
    pub fn http_status(&self) -> u16 {
        PROJECT_COPY_ERROR_HTTP_STATUS
    }

    /// Verbatim error name (`"ProjectCopyError"`).
    pub fn tag(&self) -> &'static str {
        PROJECT_COPY_ERROR_TAG
    }

    /// The `data.message` field.
    pub fn message(&self) -> &str {
        &self.data.message
    }
}

impl fmt::Display for ProjectCopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data.message)
    }
}

impl std::error::Error for ProjectCopyError {}

/// Port of the `projectCopy` params (`{ projectID: Project.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectCopyParams {
    pub projectID: schema::project_id::ProjectID,
}

/// Port of `CreatePayload` (`ProjectCopy.CreateInput` minus `projectID` and
/// `sourceDirectory`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectCopyCreatePayload {
    pub strategy: schema::project_copy::StrategyID,
    pub directory: schema::schema_primitives::AbsolutePath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Port of `RemovePayload` (`ProjectCopy.RemoveInput` minus `projectID`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectCopyRemovePayload {
    pub directory: schema::schema_primitives::AbsolutePath,
    pub force: bool,
}

/// Endpoint descriptors for `server.projectCopy`, in source order.
pub const PROJECT_COPY_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "projectCopy.create",
        openapi_identifier: "v2.projectCopy.create",
        path: "/experimental/project/:projectID/copy",
        method: HttpMethod::POST,
        summary: None,
        description: None,
        errors: &["ProjectCopyError"],
    },
    Operation {
        operation_id: "projectCopy.remove",
        openapi_identifier: "v2.projectCopy.remove",
        path: "/experimental/project/:projectID/copy",
        method: HttpMethod::DELETE,
        summary: None,
        description: None,
        errors: &["ProjectCopyError"],
    },
    Operation {
        operation_id: "projectCopy.refresh",
        openapi_identifier: "v2.projectCopy.refresh",
        path: "/experimental/project/:projectID/copy/refresh",
        method: HttpMethod::POST,
        summary: None,
        description: None,
        errors: &["ProjectCopyError"],
    },
];

/// Port of `ProjectCopyGroup` (`HttpApiGroup.make("server.projectCopy")`).
#[allow(non_upper_case_globals)]
pub const ProjectCopyGroup: Group = Group {
    name: "server.projectCopy",
    annotations: &[GroupAnnotation {
        title: Some("projectCopy"),
        description: Some("Project copy management routes."),
    }],
    operations: PROJECT_COPY_OPERATIONS,
};
