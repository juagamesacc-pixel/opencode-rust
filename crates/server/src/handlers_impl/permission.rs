//! Rust port of `packages/server/src/handlers/permission.ts` (opencode v1.18.30).
//!
//! Source 100 lines: 7 ops, `missingRequest` => PermissionNotFoundError("Permission request not found: {id}"),
//! session ownership checks, saved list filter by projectID vs location.project.id.
//!
//! PROVISIONAL: `PermissionV2`, `PermissionSaved`, `Location.Service` pending `crates/core`.

pub const GROUP: &str = "server.permission";
pub const OPERATIONS: &[&str] = &[
    "permission.request.list",
    "session.permission.create",
    "session.permission.list",
    "session.permission.get",
    "session.permission.reply",
    "permission.saved.list",
    "permission.saved.remove",
];

pub fn missing_request_message(id: &str) -> String {
    format!("Permission request not found: {id}")
}

/// Error tag verbatim: "PermissionV2.NotFoundError" vs "PermissionNotFoundError" mapping.
pub const NOT_FOUND_TAG: &str = "PermissionV2.NotFoundError";
pub const PROTOCOL_ERROR_TAG: &str = "PermissionNotFoundError";
