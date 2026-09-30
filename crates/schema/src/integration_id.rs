//! Port of `packages/schema/src/integration-id.ts`.
//!
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/error-
//! strings/keys/defaults/ordering. Source is spec.
//!
//! Source: two plain branded strings with no refinement checks and no
//! statics — `Schema.String.pipe(Schema.brand("Integration.ID"))` and
//! `Schema.String.pipe(Schema.brand("Integration.MethodID"))`. Any string is
//! accepted (no validation added here). Wire format for both is a bare string.

#![allow(non_snake_case)]

use std::ops::Deref;

/// Branded integration identifier (`IntegrationID` in `integration-id.ts`).
///
/// The `export const IntegrationID` / `export type IntegrationID` pair shares
/// one Rust item (structs are both).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IntegrationID(pub String);

impl IntegrationID {
    pub fn new(value: String) -> Self {
        IntegrationID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for IntegrationID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for IntegrationID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for IntegrationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for IntegrationID {
    fn from(value: String) -> Self {
        IntegrationID(value)
    }
}

impl From<&str> for IntegrationID {
    fn from(value: &str) -> Self {
        IntegrationID(value.to_string())
    }
}

/// Branded integration-method identifier (`IntegrationMethodID` in
/// `integration-id.ts`).
///
/// The `export const IntegrationMethodID` / `export type IntegrationMethodID`
/// pair shares one Rust item (structs are both).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IntegrationMethodID(pub String);

impl IntegrationMethodID {
    pub fn new(value: String) -> Self {
        IntegrationMethodID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for IntegrationMethodID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for IntegrationMethodID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for IntegrationMethodID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for IntegrationMethodID {
    fn from(value: String) -> Self {
        IntegrationMethodID(value)
    }
}

impl From<&str> for IntegrationMethodID {
    fn from(value: &str) -> Self {
        IntegrationMethodID(value.to_string())
    }
}
