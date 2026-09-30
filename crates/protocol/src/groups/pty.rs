//! Rust port of `packages/protocol/src/groups/pty.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `PTY_CONNECT_TICKET_QUERY`,
//! `PTY_CONNECT_TOKEN_HEADER`, `PTY_CONNECT_TOKEN_HEADER_VALUE`,
//! `hasPtyConnectTicketURL`, and the `server.pty` endpoint list (source
//! order: list, create, get, update, remove, connectToken, connect).
//!
//! The `pty.connect` query fields are decoded in the raw handler after the
//! existence check in source (a missing session responds with an empty 404
//! before any upgrade work); the OpenApi parameter merge
//! (`location[directory]`, `location[workspace]`, `cursor`, ticket) is
//! preserved below as descriptor constants.

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Verbatim from source: the ticket query parameter name (`"ticket"`).
pub const PTY_CONNECT_TICKET_QUERY: &str = "ticket";

/// Verbatim from source: the ticket header name (`"x-opencode-ticket"`).
pub const PTY_CONNECT_TOKEN_HEADER: &str = "x-opencode-ticket";

/// Verbatim from source: the ticket header value (`"1"`).
pub const PTY_CONNECT_TOKEN_HEADER_VALUE: &str = "1";

/// Query parameter names merged into the `pty.connect` OpenApi operation
/// (verbatim from source).
pub const PTY_CONNECT_QUERY_PARAMS: &[&str] = &[
    "location[directory]",
    "location[workspace]",
    "cursor",
    PTY_CONNECT_TICKET_QUERY,
];

/// Port of `hasPtyConnectTicketURL`: the authorization middleware skips
/// credential checks when this matches; the PTY connect handler is then
/// responsible for consuming and validating the ticket.
///
/// Mirrors `/^\/api\/pty\/[^/]+\/connect$/` on the pathname plus presence
/// (`searchParams.get`) of the ticket query parameter. `ticket` is
/// `Some(value)` when the `ticket` query parameter is present (an empty
/// value matches source falsy semantics and returns `false`).
#[allow(non_snake_case)]
pub fn hasPtyConnectTicketURL(pathname: &str, ticket: Option<&str>) -> bool {
    if ticket.is_none_or(str::is_empty) {
        return false;
    }
    let rest = match pathname.strip_prefix("/api/pty/") {
        Some(rest) => rest,
        None => return false,
    };
    let pty_id = match rest.strip_suffix("/connect") {
        Some(pty_id) => pty_id,
        None => return false,
    };
    !pty_id.is_empty() && !pty_id.contains('/')
}

/// Port of the pty params (`{ ptyID: Pty.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PtyParams {
    pub ptyID: schema::pty::ID,
}

/// Endpoint descriptors for `server.pty`, in source order.
pub const PTY_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "pty.list",
        openapi_identifier: "v2.pty.list",
        path: "/api/pty",
        method: HttpMethod::GET,
        summary: Some("List PTY sessions"),
        description: Some(
            "List PTY sessions for a location, including exited sessions retained until removal.",
        ),
        errors: &[],
    },
    Operation {
        operation_id: "pty.create",
        openapi_identifier: "v2.pty.create",
        path: "/api/pty",
        method: HttpMethod::POST,
        summary: Some("Create PTY session"),
        description: Some("Create a pseudo-terminal session for a location."),
        errors: &[],
    },
    Operation {
        operation_id: "pty.get",
        openapi_identifier: "v2.pty.get",
        path: "/api/pty/:ptyID",
        method: HttpMethod::GET,
        summary: Some("Get PTY session"),
        description: Some("Get one PTY session, including its exit code once exited."),
        errors: &["PtyNotFoundError"],
    },
    Operation {
        operation_id: "pty.update",
        openapi_identifier: "v2.pty.update",
        path: "/api/pty/:ptyID",
        method: HttpMethod::PUT,
        summary: Some("Update PTY session"),
        description: Some("Update the title or viewport size of one PTY session."),
        errors: &["PtyNotFoundError"],
    },
    Operation {
        operation_id: "pty.remove",
        openapi_identifier: "v2.pty.remove",
        path: "/api/pty/:ptyID",
        method: HttpMethod::DELETE,
        summary: Some("Remove PTY session"),
        description: Some("Terminate and remove one PTY session."),
        errors: &["PtyNotFoundError"],
    },
    Operation {
        operation_id: "pty.connectToken",
        openapi_identifier: "v2.pty.connectToken",
        path: "/api/pty/:ptyID/connect-token",
        method: HttpMethod::POST,
        summary: Some("Create PTY WebSocket token"),
        description: Some(
            "Create a short-lived single-use ticket for opening a PTY WebSocket connection.",
        ),
        errors: &["ForbiddenError", "PtyNotFoundError"],
    },
    Operation {
        operation_id: "pty.connect",
        openapi_identifier: "v2.pty.connect",
        path: "/api/pty/:ptyID/connect",
        method: HttpMethod::GET,
        summary: Some("Connect to PTY session"),
        description: Some(
            "Establish a WebSocket connection streaming PTY output and accepting terminal input.",
        ),
        errors: &["ForbiddenError", "PtyNotFoundError"],
    },
];

/// Port of `PtyGroup` (`HttpApiGroup.make("server.pty")`).
#[allow(non_upper_case_globals)]
pub const PtyGroup: Group = Group {
    name: "server.pty",
    annotations: &[GroupAnnotation {
        title: Some("pty"),
        description: Some("Experimental location-scoped PTY routes."),
    }],
    operations: PTY_OPERATIONS,
};
