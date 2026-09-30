//! Rust port of `packages/protocol/src/errors.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — tag strings, field names/shapes, and `httpApiStatus`
//! codes are copied verbatim from the 13 `Schema.TaggedErrorClass` definitions.
//! Field order follows source order. Optional fields omit the key when absent
//! (matching `Schema.optional` JSON encoding).
//!
//! `ProtocolError` is the `#[serde(tag = "_tag")]`-shaped union of all 13
//! variants; per-variant structs carry the fields only. Hand-rolled
//! `Display`/`Error` impls (no `thiserror`, per plan §7 default).

use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Per-variant structs (source order)
// ---------------------------------------------------------------------------

/// `InvalidRequestError` — `httpApiStatus: 400`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InvalidRequestError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

/// `UnauthorizedError` — `httpApiStatus: 401`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UnauthorizedError {
    pub message: String,
}

/// `ConflictError` — `httpApiStatus: 409`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConflictError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

/// `ServiceUnavailableError` — `httpApiStatus: 503`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceUnavailableError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// `UnknownError` — `httpApiStatus: 500`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UnknownError {
    pub message: String,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

/// `ProviderNotFoundError` — `httpApiStatus: 404`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProviderNotFoundError {
    pub providerID: String,
    pub message: String,
}

/// `SessionNotFoundError` — `httpApiStatus: 404`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionNotFoundError {
    pub sessionID: String,
    pub message: String,
}

/// `MessageNotFoundError` — `httpApiStatus: 404`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MessageNotFoundError {
    pub sessionID: String,
    pub messageID: String,
    pub message: String,
}

/// `InvalidCursorError` — `httpApiStatus: 400`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InvalidCursorError {
    pub message: String,
}

/// `PermissionNotFoundError` — `httpApiStatus: 404`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PermissionNotFoundError {
    pub requestID: String,
    pub message: String,
}

/// `QuestionNotFoundError` — `httpApiStatus: 404`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QuestionNotFoundError {
    pub requestID: String,
    pub message: String,
}

/// `ForbiddenError` — `httpApiStatus: 403`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ForbiddenError {
    pub message: String,
}

/// `PtyNotFoundError` — `httpApiStatus: 404`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PtyNotFoundError {
    pub ptyID: String,
    pub message: String,
}

// ---------------------------------------------------------------------------
// Tagged union
// ---------------------------------------------------------------------------

/// Union of the 13 protocol errors. Serializes with the verbatim `_tag`
/// discriminator alongside the variant fields, matching `TaggedErrorClass`
/// JSON (`{ _tag, ...fields }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "_tag")]
pub enum ProtocolError {
    #[serde(rename = "InvalidRequestError")]
    InvalidRequest(InvalidRequestError),
    #[serde(rename = "UnauthorizedError")]
    Unauthorized(UnauthorizedError),
    #[serde(rename = "ConflictError")]
    Conflict(ConflictError),
    #[serde(rename = "ServiceUnavailableError")]
    ServiceUnavailable(ServiceUnavailableError),
    #[serde(rename = "UnknownError")]
    Unknown(UnknownError),
    #[serde(rename = "ProviderNotFoundError")]
    ProviderNotFound(ProviderNotFoundError),
    #[serde(rename = "SessionNotFoundError")]
    SessionNotFound(SessionNotFoundError),
    #[serde(rename = "MessageNotFoundError")]
    MessageNotFound(MessageNotFoundError),
    #[serde(rename = "InvalidCursorError")]
    InvalidCursor(InvalidCursorError),
    #[serde(rename = "PermissionNotFoundError")]
    PermissionNotFound(PermissionNotFoundError),
    #[serde(rename = "QuestionNotFoundError")]
    QuestionNotFound(QuestionNotFoundError),
    #[serde(rename = "ForbiddenError")]
    Forbidden(ForbiddenError),
    #[serde(rename = "PtyNotFoundError")]
    PtyNotFound(PtyNotFoundError),
}

impl ProtocolError {
    /// Verbatim `httpApiStatus` code from source.
    pub fn http_status(&self) -> u16 {
        match self {
            ProtocolError::InvalidRequest(_) => 400,
            ProtocolError::Unauthorized(_) => 401,
            ProtocolError::Conflict(_) => 409,
            ProtocolError::ServiceUnavailable(_) => 503,
            ProtocolError::Unknown(_) => 500,
            ProtocolError::ProviderNotFound(_) => 404,
            ProtocolError::SessionNotFound(_) => 404,
            ProtocolError::MessageNotFound(_) => 404,
            ProtocolError::InvalidCursor(_) => 400,
            ProtocolError::PermissionNotFound(_) => 404,
            ProtocolError::QuestionNotFound(_) => 404,
            ProtocolError::Forbidden(_) => 403,
            ProtocolError::PtyNotFound(_) => 404,
        }
    }

    /// Verbatim `_tag` discriminator string.
    pub fn tag(&self) -> &'static str {
        match self {
            ProtocolError::InvalidRequest(_) => "InvalidRequestError",
            ProtocolError::Unauthorized(_) => "UnauthorizedError",
            ProtocolError::Conflict(_) => "ConflictError",
            ProtocolError::ServiceUnavailable(_) => "ServiceUnavailableError",
            ProtocolError::Unknown(_) => "UnknownError",
            ProtocolError::ProviderNotFound(_) => "ProviderNotFoundError",
            ProtocolError::SessionNotFound(_) => "SessionNotFoundError",
            ProtocolError::MessageNotFound(_) => "MessageNotFoundError",
            ProtocolError::InvalidCursor(_) => "InvalidCursorError",
            ProtocolError::PermissionNotFound(_) => "PermissionNotFoundError",
            ProtocolError::QuestionNotFound(_) => "QuestionNotFoundError",
            ProtocolError::Forbidden(_) => "ForbiddenError",
            ProtocolError::PtyNotFound(_) => "PtyNotFoundError",
        }
    }

    /// The `message` field carried by every variant.
    pub fn message(&self) -> &str {
        match self {
            ProtocolError::InvalidRequest(e) => &e.message,
            ProtocolError::Unauthorized(e) => &e.message,
            ProtocolError::Conflict(e) => &e.message,
            ProtocolError::ServiceUnavailable(e) => &e.message,
            ProtocolError::Unknown(e) => &e.message,
            ProtocolError::ProviderNotFound(e) => &e.message,
            ProtocolError::SessionNotFound(e) => &e.message,
            ProtocolError::MessageNotFound(e) => &e.message,
            ProtocolError::InvalidCursor(e) => &e.message,
            ProtocolError::PermissionNotFound(e) => &e.message,
            ProtocolError::QuestionNotFound(e) => &e.message,
            ProtocolError::Forbidden(e) => &e.message,
            ProtocolError::PtyNotFound(e) => &e.message,
        }
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ProtocolError {}
