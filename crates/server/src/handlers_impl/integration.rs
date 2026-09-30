//! Rust port of `packages/server/src/handlers/integration.ts` (opencode v1.18.30).
//!
//! Source 77 lines: 7 ops under `server.integration`, authorize helper mapping
//! `Integration.AuthorizationError` -> `InvalidRequestError(message:"Authentication failed", kind:"integration_authorization")`,
//! plus code-required mapping.
//!
//! PROVISIONAL: `Integration.Service` pending `crates/core`.

pub const GROUP: &str = "server.integration";
pub const OPERATIONS: &[&str] = &[
    "integration.list",
    "integration.get",
    "integration.connect.key",
    "integration.connect.oauth",
    "integration.attempt.status",
    "integration.attempt.complete",
    "integration.attempt.cancel",
];

pub const AUTH_FAILED_MESSAGE: &str = "Authentication failed";
pub const AUTH_FAILED_KIND: &str = "integration_authorization";
pub const CODE_REQUIRED_MESSAGE: &str = "Authorization code is required";
pub const CODE_REQUIRED_KIND: &str = "integration_code_required";

pub fn auth_failed_error() -> (String, String) {
    (
        AUTH_FAILED_MESSAGE.to_string(),
        AUTH_FAILED_KIND.to_string(),
    )
}

pub fn code_required_or_auth_failed(is_code_required: bool) -> (String, String) {
    if is_code_required {
        (
            CODE_REQUIRED_MESSAGE.to_string(),
            CODE_REQUIRED_KIND.to_string(),
        )
    } else {
        (
            AUTH_FAILED_MESSAGE.to_string(),
            AUTH_FAILED_KIND.to_string(),
        )
    }
}
