//! Rust port of `packages/protocol/src/groups/model.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `model.list` (`GET /api/model`).

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Endpoint descriptors for `server.model`, in source order.
pub const MODEL_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "model.list",
    openapi_identifier: "v2.model.list",
    path: "/api/model",
    method: HttpMethod::GET,
    summary: Some("List models"),
    description: Some("Retrieve available models ordered by release date."),
    errors: &["ServiceUnavailableError"],
}];

/// Port of `ModelGroup` (`HttpApiGroup.make("server.model")`).
#[allow(non_upper_case_globals)]
pub const ModelGroup: Group = Group {
    name: "server.model",
    annotations: &[GroupAnnotation {
        title: Some("models"),
        description: Some("Experimental model routes."),
    }],
    operations: MODEL_OPERATIONS,
};
