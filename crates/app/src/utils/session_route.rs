//! Rust port of `packages/app/src/utils/session-route.ts` (opencode v1.18.30).
//!
//! Source 39 lines: `sessionHref`, `legacySessionHref`, `requireServerKey`,
//! `legacySessionServer`, `rootSession`.
//!
//! 1:1 notes:
//! - `base64Encode` from `@opencode-ai/core/util/encode` is modelled by
//!   `encode_padless` (padding stripped; matches `aHR0cHM6Ly9leGFtcGxlLmNvbTo0MDk2`
//!   and `L1VzZXJzL2V4YW1wbGUvcHJvamVjdA` in the 1:1 tests).
//! - `ServerConnection.Key`/`Key.make` are identity on strings (the
//!   `context/server` stub has an empty `Key` struct).
//! - `rootSession`'s async `get` is modelled as a closure returning
//!   `Result<T, String>` (the source rejects with `Error`).
//! - Original file: `packages/app/src/utils/session-route.ts`

#![allow(dead_code)]

use crate::utils::base64::{decode64, encode_padless};
use std::collections::HashSet;
use std::fmt;

/// Mirrors `sessionHref(server, sessionID)` → `/server/<base64>/session/<id>`.
pub fn session_href(server: &str, session_id: &str) -> String {
    format!("/server/{}/session/{}", encode_padless(server), session_id)
}

/// Mirrors `legacySessionHref(directory, sessionID)` → `/<base64>/session/<id>`.
pub fn legacy_session_href(directory: &str, session_id: &str) -> String {
    format!("/{}/session/{}", encode_padless(directory), session_id)
}

/// Mirrors `new Error("Invalid server route")`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRouteError(pub String);

impl fmt::Display for SessionRouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SessionRouteError {}

/// Mirrors `requireServerKey(segment)` — throws on undecodable/mismatched
/// input; returns the decoded key (identity `Key.make(key)`).
pub fn require_server_key(segment: Option<&str>) -> Result<String, SessionRouteError> {
    let key = match decode64(segment) {
        Some(key) => key,
        None => return Err(SessionRouteError("Invalid server route".to_string())),
    };
    if encode_padless(&key) != segment.unwrap_or_default() {
        return Err(SessionRouteError("Invalid server route".to_string()));
    }
    Ok(key)
}

/// Mirrors the `{ type: "session"; server; sessionId }` tab shape (roles are
/// not needed by this helper).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionTab {
    pub server: String,
    pub session_id: String,
}

/// Mirrors `legacySessionServer(tabs, sessionID, active)`.
pub fn legacy_session_server(tabs: &[SessionTab], session_id: &str, active: &str) -> String {
    let matches: Vec<&SessionTab> = tabs
        .iter()
        .filter(|tab| tab.session_id == session_id)
        .collect();
    match matches.iter().find(|tab| tab.server == active) {
        Some(tab) => tab.server.clone(),
        None if matches.len() == 1 => matches[0].server.clone(),
        None => active.to_string(),
    }
}

/// Mirrors the `SessionParent` constraint: `{ id: string; parentID?: string }`.
pub trait SessionParent {
    fn id(&self) -> &str;
    fn parent_id(&self) -> Option<&str>;
}

/// Mirrors `rootSession<T>(session, get)` — walks `parentID` links, rejecting
/// on cycles. The async fetch is modelled by the `get` closure.
pub fn root_session<T: SessionParent + Clone>(
    session: &T,
    get: &mut dyn FnMut(String) -> Result<T, String>,
) -> Result<T, String> {
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(session.id().to_string());
    let mut current = session.clone();
    while let Some(parent_id) = current.parent_id() {
        if seen.contains(parent_id) {
            return Err(format!("Session parent cycle: {parent_id}"));
        }
        seen.insert(parent_id.to_string());
        current = get(parent_id.to_string())?;
    }
    Ok(current)
}
