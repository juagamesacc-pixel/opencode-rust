//! Port of `packages/schema/src/session-id.ts`.
//!
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/error-
//! strings/keys/defaults/ordering. Source is spec.
//!
//! Source: `Schema.String` checked with `Schema.isStartsWith("ses")` (no
//! underscore — `"sesame"` passes, exactly as in source; not tightened),
//! branded `"SessionID"`, with
//! `statics((schema) => ({ create, descending }))` where
//! `create = () => schema.make("ses_" + descending())` and
//! `descending = (id?: string) => (id === undefined ? create() : schema.make(id))`.
//! Like `Schema.make`, a provided `id` is wrapped without revalidation.

#![allow(non_snake_case)]

use std::ops::Deref;

/// Branded session identifier (`SessionID` in `session-id.ts`).
///
/// Wire format is a bare string. The `export const SessionID` /
/// `export type SessionID` pair shares one Rust item (structs are both).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SessionID(pub String);

impl SessionID {
    /// Port of `create` (`() => schema.make("ses_" + descending())`).
    pub fn create() -> Self {
        SessionID(format!("ses_{}", crate::identifier::descending()))
    }

    /// Port of `descending` (`(id?: string) => id === undefined ? create() : schema.make(id)`).
    ///
    /// `None` creates a fresh ID; `Some(id)` wraps as-is (strict `undefined`
    /// check in source — even `""` is wrapped, not regenerated).
    pub fn descending(id: Option<&str>) -> Self {
        match id {
            None => Self::create(),
            Some(id) => SessionID(id.to_string()),
        }
    }

    /// Port of the `Schema.isStartsWith("ses")` check.
    pub fn is_valid(value: &str) -> bool {
        value.starts_with("ses")
    }

    pub fn new(value: String) -> Self {
        SessionID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for SessionID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SessionID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SessionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SessionID {
    fn from(value: String) -> Self {
        SessionID(value)
    }
}

impl From<&str> for SessionID {
    fn from(value: &str) -> Self {
        SessionID(value.to_string())
    }
}
