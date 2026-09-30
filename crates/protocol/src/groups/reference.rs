//! Rust port of `packages/protocol/src/groups/reference.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `reference.list` (`GET /api/reference`).

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Endpoint descriptors for `server.reference`, in source order.
pub const REFERENCE_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "reference.list",
    openapi_identifier: "v2.reference.list",
    path: "/api/reference",
    method: HttpMethod::GET,
    summary: Some("List references"),
    description: Some("List references available in the requested location."),
    errors: &[],
}];

/// Port of `ReferenceGroup` (`HttpApiGroup.make("server.reference")`).
#[allow(non_upper_case_globals)]
pub const ReferenceGroup: Group = Group {
    name: "server.reference",
    annotations: &[GroupAnnotation {
        title: Some("reference"),
        description: Some("Location-scoped project references."),
    }],
    operations: REFERENCE_OPERATIONS,
};
