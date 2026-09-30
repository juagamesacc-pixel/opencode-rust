//! Rust port of `packages/protocol/src/groups/command.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `command.list` (`GET /api/command`).

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Endpoint descriptors for `server.command`, in source order.
pub const COMMAND_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "command.list",
    openapi_identifier: "v2.command.list",
    path: "/api/command",
    method: HttpMethod::GET,
    summary: Some("List commands"),
    description: Some("Retrieve currently registered commands."),
    errors: &[],
}];

/// Port of `CommandGroup` (`HttpApiGroup.make("server.command")`).
#[allow(non_upper_case_globals)]
pub const CommandGroup: Group = Group {
    name: "server.command",
    annotations: &[GroupAnnotation {
        title: Some("commands"),
        description: Some("Experimental command routes."),
    }],
    operations: COMMAND_OPERATIONS,
};
