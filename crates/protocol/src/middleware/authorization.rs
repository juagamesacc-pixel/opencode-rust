//! Rust port of `packages/protocol/src/middleware/authorization.ts`
//! (opencode v1.18.30).
//!
//! 1:1 exact translation — service id string copied verbatim. Types +
//! constants only; HTTP serving lives elsewhere. The middleware's error
//! channel is `UnauthorizedError` (see `crate::errors`).

/// Service id verbatim from source: `"@opencode/HttpApiAuthorization"`.
pub const AUTHORIZATION_SERVICE_ID: &str = "@opencode/HttpApiAuthorization";

/// Descriptor for the `Authorization` HttpApi middleware.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Authorization;

impl Authorization {
    /// The middleware service id (`"@opencode/HttpApiAuthorization"`).
    pub const fn service_id() -> &'static str {
        AUTHORIZATION_SERVICE_ID
    }

    /// The `_tag` of the middleware error (`UnauthorizedError`).
    pub const fn error_tag() -> &'static str {
        "UnauthorizedError"
    }
}
