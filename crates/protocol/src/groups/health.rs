//! Rust port of `packages/protocol/src/groups/health.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `health.get` (`GET /api/health`, identifier
//! `v2.health.get`).

use crate::api::{Group, HttpMethod, Operation};

/// Endpoint descriptors for `server.health`, in source order.
pub const HEALTH_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "health.get",
    openapi_identifier: "v2.health.get",
    path: "/api/health",
    method: HttpMethod::GET,
    summary: Some("Check server health"),
    description: Some("Check whether the API server is ready to accept requests."),
    errors: &[],
}];

/// Port of `HealthGroup` (`HttpApiGroup.make("server.health")`).
#[allow(non_upper_case_globals)]
pub const HealthGroup: Group = Group {
    name: "server.health",
    annotations: &[],
    operations: HEALTH_OPERATIONS,
};
