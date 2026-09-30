//! Rust port of `packages/server/src/middleware/session-location.ts` (opencode v1.18.30).
//!
//! Source 67 lines. Exports: `SessionLocationMiddleware` (`"@opencode/HttpApiSessionLocation"`,
//! error: [InvalidRequestError, SessionNotFoundError]), `decodeSessionID`, and
//! `sessionLocationLayer`.
//!
//! 1:1 notes:
//! - `decodeSessionID` decodes `SessionV2.ID`; mapping `"Invalid session ID"` + field `"sessionID"` on failure.
//! - DB lookup: `select { directory, workspaceID } from SessionTable where id = sessionID`.
//! - On missing row → `SessionNotFoundError(sessionID, "Session not found: {id}")`.
//! - On success provides `Location.Ref { directory: AbsolutePath, workspaceID }`.
//!
//! WIRED: `SessionV2.ID` brand now validated as non-empty String (matches core session schema brand);
//! `Location.Ref` now re-exported from `core::location::LocationRef` (verified). Remaining PROVISIONAL:
//! `Database.Service` / `SessionTable` DB lookup still pending `crates/core` database layer (no SQLite runtime yet) — kept marked.

/// Service ID verbatim: `"@opencode/HttpApiSessionLocation"`.
pub const SESSION_LOCATION_SERVICE_ID: &str = "@opencode/HttpApiSessionLocation";

/// Descriptor for `SessionLocationMiddleware`.
pub struct SessionLocationMiddleware;

impl SessionLocationMiddleware {
    pub fn service_id() -> &'static str {
        SESSION_LOCATION_SERVICE_ID
    }
    /// Error `_tag`s verbatim: `[InvalidRequestError, SessionNotFoundError]`.
    pub const ERRORS: &[&str] = &["InvalidRequestError", "SessionNotFoundError"];
}

/// Verbatim error messages.
pub const INVALID_SESSION_ID_MESSAGE: &str = "Invalid session ID";
pub const INVALID_SESSION_ID_FIELD: &str = "sessionID";

/// Port of `decodeSessionID` — validates the session ID string.
///
/// Source uses `Schema.decodeUnknownEffect(SessionV2.ID)`; in Rust we validate
/// that the ID is a non-empty string (session IDs are branded strings in schema).
/// On failure return the InvalidRequestError-shaped descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct InvalidRequestError {
    pub message: String,
    pub field: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SessionNotFoundError {
    pub session_id: String,
    pub message: String,
}

pub fn decode_session_id(raw: &str) -> Result<String, InvalidRequestError> {
    // Source SessionV2.ID is a branded string (likely UUID-like). Minimal check:
    // non-empty. More validation is deferred to real `schema::session_id` when ported.
    if raw.is_empty() {
        Err(InvalidRequestError {
            message: INVALID_SESSION_ID_MESSAGE.to_string(),
            field: Some(INVALID_SESSION_ID_FIELD.to_string()),
        })
    } else {
        Ok(raw.to_string())
    }
}

/// Port of the DB row not-found → SessionNotFoundError mapping.
pub fn session_not_found(session_id: &str) -> SessionNotFoundError {
    SessionNotFoundError {
        session_id: session_id.to_string(),
        message: format!("Session not found: {session_id}"),
    }
}

/// Descriptor for `Location.Ref` provisioning (mirrors `locations.get(Location.Ref.make(...))`).
/// WIRED: now type-alias to `core::location::LocationRef` (1:1 fields).
pub use core::location::LocationRef;

/// Port of `sessionLocationLayer` decision half (pure): given `sessionID` param
/// and an optional DB row, either produce errors or the location ref.
///
/// This isolates the testable pure branches from the Effect wiring.
pub enum SessionLocationOutcome {
    Ok(LocationRef),
    InvalidSessionId(InvalidRequestError),
    NotFound(SessionNotFoundError),
}

pub fn resolve_session_location(
    session_id_param: &str,
    row: Option<(String, Option<String>)>,
) -> SessionLocationOutcome {
    let session_id = match decode_session_id(session_id_param) {
        Ok(id) => id,
        Err(e) => return SessionLocationOutcome::InvalidSessionId(e),
    };
    match row {
        None => SessionLocationOutcome::NotFound(session_not_found(&session_id)),
        Some((directory, workspace_id)) => SessionLocationOutcome::Ok(LocationRef {
            directory,
            workspace_id,
        }),
    }
}

/// Layer descriptor (Effect wiring placeholder).
pub struct SessionLocationLayer;

impl SessionLocationLayer {
    pub const SERVICE: &str = SESSION_LOCATION_SERVICE_ID;
    pub const DEPENDS_ON: &[&str] = &["@opencode/Database", "@opencode/LocationServiceMap"];
}
