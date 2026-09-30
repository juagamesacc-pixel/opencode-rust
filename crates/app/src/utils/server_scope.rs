//! Rust port of `packages/app/src/utils/server-scope.ts` (opencode v1.18.30).
//!
//! Source 73 lines. Branded string types (`ServerScope`, `SessionRouteKey`,
//! `SessionStateKey`, `ScopedKey`) are plain `String` here; the four const
//! namespaces (`ServerScope`, `SessionRouteKey`, `SessionStateKey`, `ScopedKey`)
//! become free functions grouped per namespace with the object keys mirrored
//! in the function names (`ServerScope.local` → `server_scope_local`,
//! `ScopedKey.from` → `scoped_key_from`, ...).
//!
//! 1:1 notes:
//! - `fragment` throwing on null bytes is modelled as `Result<String, ScopeError>`
//!   with the same message text ("Server scope cannot contain null bytes",
//!   "Scoped key part cannot contain null bytes", ...).
//! - `ServerScope.fromServerKey` compares against the `"sidecar"` sentinel and
//!   an optional canonical local server key (both strings in this port).
//! - `migrateLegacySessionStateKeys` operates on `serde_json::Value` (it is a
//!   JSON-shape transform in the source).
//! - Original file: `packages/app/src/utils/server-scope.ts`

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

/// Mirrors `type ServerScope = string & { readonly __brand: "ServerScope" }`.
pub type ServerScope = String;
/// Mirrors `type SessionRouteKey = ...`.
pub type SessionRouteKey = String;
/// Mirrors `type SessionStateKey = ...`.
pub type SessionStateKey = String;
/// Mirrors `type ScopedKey = ...`.
pub type ScopedKey = String;

const SEPARATOR: &str = "\u{0}";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeError(pub String);

impl fmt::Display for ScopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ScopeError {}

fn fragment(label: &str, value: &str) -> Result<String, ScopeError> {
    if value.contains(SEPARATOR) {
        Err(ScopeError(format!("{label} cannot contain null bytes")))
    } else {
        Ok(value.to_string())
    }
}

fn compose(scope: &str, parts: &[String]) -> Result<String, ScopeError> {
    let mut pieces = vec![fragment("Server scope", scope)?];
    for part in parts {
        pieces.push(fragment("Scoped key part", part)?);
    }
    Ok(pieces.join(SEPARATOR))
}

/// `ServerScope.local`
pub const SERVER_SCOPE_LOCAL: &str = "local";

/// `ServerScope.fromServerKey(key, canonicalLocalServer?)`
pub fn server_scope_from_server_key(
    key: &str,
    canonical_local_server: Option<&str>,
) -> Result<ServerScope, ScopeError> {
    let scope = if key == "sidecar" || Some(key) == canonical_local_server {
        SERVER_SCOPE_LOCAL
    } else {
        key
    };
    fragment("Server scope", scope)
}

/// `SessionRouteKey.fromRoute(dir?, sessionID?)`
pub fn session_route_key_from_route(
    dir: Option<&str>,
    session_id: Option<&str>,
) -> Result<SessionRouteKey, ScopeError> {
    let value = match session_id {
        Some(session_id) => format!("{}/{}", dir.unwrap_or_default(), session_id),
        None => dir.unwrap_or_default().to_string(),
    };
    fragment("Session route", &value)
}

/// `SessionRouteKey.fromLegacy(key)`
pub fn session_route_key_from_legacy(key: &str) -> Result<SessionRouteKey, ScopeError> {
    fragment("Legacy session route", key)
}

/// `SessionStateKey.from(scope, route)`
pub fn session_state_key_from(
    scope: &ServerScope,
    route: &SessionRouteKey,
) -> Result<SessionStateKey, ScopeError> {
    compose(scope, std::slice::from_ref(route))
}

/// `SessionStateKey.route(key)` — never fails (the tail after the last
/// separator, or a separator-free key, cannot contain null bytes).
pub fn session_state_key_route(key: &str) -> SessionRouteKey {
    let split = key.rfind(SEPARATOR);
    let tail = match split {
        Some(split) => &key[split + SEPARATOR.len()..],
        None => key,
    };
    session_route_key_from_legacy(tail)
        .unwrap_or_else(|err| unreachable!("tail of {key:?} cannot contain null bytes: {err}"))
}

/// `SessionStateKey.scope(key)`
pub fn session_state_key_scope(key: &str) -> Result<ServerScope, ScopeError> {
    let Some(split) = key.find(SEPARATOR) else {
        return Ok(SERVER_SCOPE_LOCAL.to_string());
    };
    fragment("Stored server scope", &key[..split])
}

/// `ScopedKey.from(scope, ...parts)`
pub fn scoped_key_from(scope: &ServerScope, parts: &[String]) -> Result<ScopedKey, ScopeError> {
    compose(scope, parts)
}

/// `ScopedKey.prefix(scope, ...parts)`
pub fn scoped_key_prefix(scope: &ServerScope, parts: &[String]) -> Result<String, ScopeError> {
    Ok(format!("{}{}", scoped_key_from(scope, parts)?, SEPARATOR))
}

/// Mirrors `migrateLegacySessionStateKeys(value: unknown)`.
pub fn migrate_legacy_session_state_keys(value: &Value) -> Value {
    let Some(object) = value.as_object() else {
        return value.clone();
    };
    if object.keys().all(|key| key.contains(SEPARATOR)) {
        return value.clone();
    }
    let mut scoped: HashMap<String, Value> = HashMap::new();
    for (key, item) in object.iter().filter(|(key, _)| key.contains(SEPARATOR)) {
        scoped.insert(key.clone(), item.clone());
    }
    for (key, item) in object.iter().filter(|(key, _)| !key.contains(SEPARATOR)) {
        let next = format!("{SERVER_SCOPE_LOCAL}{SEPARATOR}{key}");
        scoped.entry(next).or_insert_with(|| item.clone());
    }
    serde_json::to_value(scoped).unwrap_or(Value::Null)
}
