//! Rust port of `packages/protocol/src/groups/integration.ts`
//! (opencode v1.18.30).
//!
//! 1:1 exact translation — the `server.integration` endpoint list
//! (operation IDs, `/api/…` paths, methods, OpenApi identifiers/summaries/
//! descriptions, and error channels byte-identical, in source order).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Port of the integration params (`{ integrationID: Integration.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntegrationParams {
    pub integrationID: schema::integration::ID,
}

/// Port of the attempt params (`{ attemptID: Integration.AttemptID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntegrationAttemptParams {
    pub attemptID: schema::integration::AttemptID,
}

/// Port of the `integration.connect.key` payload
/// (`{ key: string, label (optional) }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntegrationConnectKeyPayload {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Port of the `integration.connect.oauth` payload
/// (`{ methodID: Integration.MethodID, inputs: Record<string, string>,
/// label (optional) }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntegrationConnectOAuthPayload {
    pub methodID: schema::integration::MethodID,
    pub inputs: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Port of the `integration.attempt.complete` payload
/// (`{ code (optional) }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntegrationAttemptCompletePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Endpoint descriptors for `server.integration`, in source order.
pub const INTEGRATION_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "integration.list",
        openapi_identifier: "v2.integration.list",
        path: "/api/integration",
        method: HttpMethod::GET,
        summary: Some("List integrations"),
        description: Some("Retrieve available integrations and their authentication methods."),
        errors: &[],
    },
    Operation {
        operation_id: "integration.get",
        openapi_identifier: "v2.integration.get",
        path: "/api/integration/:integrationID",
        method: HttpMethod::GET,
        summary: Some("Get integration"),
        description: Some("Retrieve one integration and its authentication methods."),
        errors: &[],
    },
    Operation {
        operation_id: "integration.connect.key",
        openapi_identifier: "v2.integration.connect.key",
        path: "/api/integration/:integrationID/connect/key",
        method: HttpMethod::POST,
        summary: Some("Connect with key"),
        description: Some("Run a key authentication method and store the resulting credential."),
        errors: &["InvalidRequestError"],
    },
    Operation {
        operation_id: "integration.connect.oauth",
        openapi_identifier: "v2.integration.connect.oauth",
        path: "/api/integration/:integrationID/connect/oauth",
        method: HttpMethod::POST,
        summary: Some("Begin OAuth connection"),
        description: Some("Start an OAuth attempt and return the authorization details."),
        errors: &["InvalidRequestError"],
    },
    Operation {
        operation_id: "integration.attempt.status",
        openapi_identifier: "v2.integration.attempt.status",
        path: "/api/integration/attempt/:attemptID",
        method: HttpMethod::GET,
        summary: Some("Get OAuth attempt status"),
        description: Some("Poll the current status of an OAuth attempt."),
        errors: &[],
    },
    Operation {
        operation_id: "integration.attempt.complete",
        openapi_identifier: "v2.integration.attempt.complete",
        path: "/api/integration/attempt/:attemptID/complete",
        method: HttpMethod::POST,
        summary: Some("Complete OAuth connection"),
        description: Some(
            "Complete a code-based OAuth attempt and store the resulting credential.",
        ),
        errors: &["InvalidRequestError"],
    },
    Operation {
        operation_id: "integration.attempt.cancel",
        openapi_identifier: "v2.integration.attempt.cancel",
        path: "/api/integration/attempt/:attemptID",
        method: HttpMethod::DELETE,
        summary: Some("Cancel OAuth connection"),
        description: Some("Cancel an OAuth attempt and release its resources."),
        errors: &[],
    },
];

/// Port of `IntegrationGroup` (`HttpApiGroup.make("server.integration")`).
#[allow(non_upper_case_globals)]
pub const IntegrationGroup: Group = Group {
    name: "server.integration",
    annotations: &[GroupAnnotation {
        title: Some("integrations"),
        description: Some("Integration discovery and authentication routes."),
    }],
    operations: INTEGRATION_OPERATIONS,
};
