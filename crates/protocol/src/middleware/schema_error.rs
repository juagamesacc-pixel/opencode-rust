//! Rust port of `packages/protocol/src/middleware/schema-error.ts`
//! (opencode v1.18.30).
//!
//! 1:1 exact translation — service id string copied verbatim. Types +
//! constants only; HTTP serving lives elsewhere. The middleware's error
//! channel is `InvalidRequestError` (see `crate::errors`).

/// Service id verbatim from source: `"@opencode/HttpApiSchemaError"`.
pub const SCHEMA_ERROR_SERVICE_ID: &str = "@opencode/HttpApiSchemaError";

/// Descriptor for the `SchemaErrorMiddleware` HttpApi middleware.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaErrorMiddleware;

impl SchemaErrorMiddleware {
    /// The middleware service id (`"@opencode/HttpApiSchemaError"`).
    pub const fn service_id() -> &'static str {
        SCHEMA_ERROR_SERVICE_ID
    }

    /// The `_tag` of the middleware error (`InvalidRequestError`).
    pub const fn error_tag() -> &'static str {
        "InvalidRequestError"
    }
}
