//! Rust port of `packages/server/src/middleware/schema-error.ts` (opencode v1.18.30).
//!
//! Source 20 lines. Re-exports `SchemaErrorMiddleware`, defines
//! `REASON_LIMIT=1024`, `truncateReason`, and `schemaErrorLayer`.
//!
//! 1:1 notes:
//! - `REASON_LIMIT = 1024` verbatim.
//! - `truncateReason` slices to 1024 + `... (N more chars)` verbatim.
//! - `schemaErrorLayer` transforms schema rejections to `InvalidRequestError` with
//!   `message: truncated reason, kind`.

/// Truncation limit verbatim: `1024`.
pub const REASON_LIMIT: usize = 1024;

/// Port of `function truncateReason(reason)` — caps at 1024 chars.
pub fn truncate_reason(reason: &str) -> String {
    if reason.len() <= REASON_LIMIT {
        return reason.to_string();
    }
    let extra = reason.len() - REASON_LIMIT;
    format!("{}... ({} more chars)", &reason[..REASON_LIMIT], extra)
}

/// Port of the schema-error transform: logs warning with `kind` + `reason`,
/// then fails `InvalidRequestError(message: truncated, kind)`.
///
/// Pure data transform (no Effect runtime).
#[derive(Clone, Debug, PartialEq)]
pub struct InvalidRequestError {
    pub message: String,
    pub kind: Option<String>,
}

pub fn transform_schema_error(kind: &str, reason: &str) -> InvalidRequestError {
    let truncated = truncate_reason(reason);
    // Mirrors `Effect.logWarning("schema rejection").pipe(annotateLogs({kind, reason: truncated}))`
    // The log side-effect is noted but not performed in this pure layer; caller logs if needed.
    InvalidRequestError {
        message: truncated,
        kind: Some(kind.to_string()),
    }
}

/// Service ID verbatim from protocol: `"@opencode/HttpApiSchemaError"` (if defined).
/// Re-exported descriptor.
pub use protocol::middleware::schema_error::SchemaErrorMiddleware;

/// Descriptor for `schemaErrorLayer`.
pub struct SchemaErrorLayer;

impl SchemaErrorLayer {
    pub const SERVICE: &str = protocol::middleware::schema_error::SCHEMA_ERROR_SERVICE_ID;
}
