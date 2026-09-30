//! Rust port of `packages/protocol/src/groups/permission.ts`
//! (opencode v1.18.30).
//!
//! 1:1 exact translation — the `makePermissionGroup` factory endpoint list
//! (operation IDs, `/api/…` paths, methods, OpenApi identifiers/summaries/
//! descriptions, and error channels byte-identical, in source order).
//!
//! Source note preserved: Effect applies group middleware only to endpoints
//! already added, so the first three endpoints carry the location middleware
//! while the session endpoints carry the session-location middleware. Both
//! middleware keys are server-side placement with no descriptor
//! representation here.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Port of the `permission.saved.list` query
/// (`{ projectID: Project.ID (optional) }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PermissionSavedQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectID: Option<schema::project_id::ProjectID>,
}

/// Port of the `permission.saved.remove` params (`{ id: PermissionSaved.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SavedPermissionParams {
    pub id: schema::permission_saved::ID,
}

/// Port of the session permission params
/// (`{ sessionID: Session.ID, requestID: Permission.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionPermissionParams {
    pub sessionID: schema::session_id::SessionID,
    pub requestID: schema::permission::ID,
}

/// Port of the `session.permission.create` payload (`id`/`agent` optional;
/// `action`/`resources`/`save`/`metadata`/`source` projected from
/// `Permission.Request` fields).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionPermissionCreatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<schema::permission::ID>,
    pub action: String,
    pub resources: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<schema::permission::Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<schema::agent::ID>,
}

/// Port of the `session.permission.reply` payload
/// (`{ reply: Permission.Reply, message (optional) }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionPermissionReplyPayload {
    pub reply: schema::permission::Reply,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Endpoint descriptors for `server.permission`, in source order.
pub const PERMISSION_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "permission.request.list",
        openapi_identifier: "v2.permission.request.list",
        path: "/api/permission/request",
        method: HttpMethod::GET,
        summary: Some("List pending permission requests"),
        description: Some("Retrieve pending permission requests for a location."),
        errors: &[],
    },
    Operation {
        operation_id: "permission.saved.list",
        openapi_identifier: "v2.permission.saved.list",
        path: "/api/permission/saved",
        method: HttpMethod::GET,
        summary: Some("List saved permissions"),
        description: Some("Retrieve saved permissions, optionally filtered by project."),
        errors: &[],
    },
    Operation {
        operation_id: "permission.saved.remove",
        openapi_identifier: "v2.permission.saved.remove",
        path: "/api/permission/saved/:id",
        method: HttpMethod::DELETE,
        summary: Some("Remove saved permission"),
        description: Some("Remove a saved permission by ID."),
        errors: &[],
    },
    Operation {
        operation_id: "session.permission.create",
        openapi_identifier: "v2.session.permission.create",
        path: "/api/session/:sessionID/permission",
        method: HttpMethod::POST,
        summary: Some("Create permission request"),
        description: Some(
            "Evaluate and, when approval is required, create a permission request for a session.",
        ),
        errors: &["SessionNotFoundError"],
    },
    Operation {
        operation_id: "session.permission.list",
        openapi_identifier: "v2.session.permission.list",
        path: "/api/session/:sessionID/permission",
        method: HttpMethod::GET,
        summary: Some("List session permission requests"),
        description: Some("Retrieve pending permission requests owned by a session."),
        errors: &["SessionNotFoundError"],
    },
    Operation {
        operation_id: "session.permission.get",
        openapi_identifier: "v2.session.permission.get",
        path: "/api/session/:sessionID/permission/:requestID",
        method: HttpMethod::GET,
        summary: Some("Get permission request"),
        description: Some("Retrieve a pending permission request owned by a session."),
        errors: &["SessionNotFoundError", "PermissionNotFoundError"],
    },
    Operation {
        operation_id: "session.permission.reply",
        openapi_identifier: "v2.session.permission.reply",
        path: "/api/session/:sessionID/permission/:requestID/reply",
        method: HttpMethod::POST,
        summary: Some("Reply to pending permission request"),
        description: Some("Respond to a pending permission request owned by a session."),
        errors: &["SessionNotFoundError", "PermissionNotFoundError"],
    },
];

/// Group name verbatim from source (`HttpApiGroup.make("server.permission")`).
pub const PERMISSION_GROUP_NAME: &str = "server.permission";

/// Port of `makePermissionGroup(locationMiddleware,
/// sessionLocationMiddleware)`. Middleware keys are server-side placement
/// (no descriptor representation); endpoint middleware assignment follows the
/// source note above. Returns the `server.permission` group descriptor.
pub fn make_permission_group() -> Group {
    Group {
        name: PERMISSION_GROUP_NAME,
        annotations: &[GroupAnnotation {
            title: Some("permissions"),
            description: Some("Experimental permission routes."),
        }],
        operations: PERMISSION_OPERATIONS,
    }
}
