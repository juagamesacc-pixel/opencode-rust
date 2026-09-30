//! Rust port of `packages/protocol/src/groups/agent.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `agent.list` (`GET /api/agent`).

use crate::api::{Group, HttpMethod, Operation};

/// Endpoint descriptors for `server.agent`, in source order.
pub const AGENT_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "agent.list",
    openapi_identifier: "v2.agent.list",
    path: "/api/agent",
    method: HttpMethod::GET,
    summary: Some("List agents"),
    description: Some("Retrieve currently registered agents."),
    errors: &[],
}];

/// Port of `AgentGroup` (`HttpApiGroup.make("server.agent")`).
#[allow(non_upper_case_globals)]
pub const AgentGroup: Group = Group {
    name: "server.agent",
    annotations: &[],
    operations: AGENT_OPERATIONS,
};
