//! Rust port of `packages/schema/src/credential.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! `Value` is `Union([OAuth, Key]).pipe(Schema.toTaggedUnion("type"))` — an
//! internally tagged union on `type`; the enum carries inline variant fields
//! (the `type` tag key is supplied by serde), matching the key shape of the
//! exported `OAuth` / `Key` structs.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Deref;

/// Brand `Credential.ID` — transparent newtype, wire format stays bare string.
///
/// No refinement check in source; `statics` provides `create()`
/// (`() => schema.make("cred_" + ascending())`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Port of `create` (`() => schema.make("cred_" + ascending())`).
    pub fn create() -> Self {
        ID(format!("cred_{}", crate::identifier::ascending()))
    }

    pub fn new(value: String) -> Self {
        ID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for ID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ID {
    fn from(value: String) -> Self {
        ID(value)
    }
}

impl From<&str> for ID {
    fn from(value: &str) -> Self {
        ID(value.to_string())
    }
}

/// Identifier `Credential.OAuth`. Field order is verbatim:
/// type, methodID, refresh, access, expires, metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth {
    #[serde(rename = "type")]
    pub r#type: String,
    pub methodID: crate::integration_id::IntegrationMethodID,
    pub refresh: String,
    pub access: String,
    pub expires: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
}

impl OAuth {
    /// Verbatim literal of the `type` key.
    pub const TYPE: &'static str = "oauth";
}

/// Identifier `Credential.Key`. Field order is verbatim: type, key, metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[serde(rename = "type")]
    pub r#type: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
}

impl Key {
    /// Verbatim literal of the `type` key.
    pub const TYPE: &'static str = "key";
}

/// Identifier `Credential.Value` — `Union([OAuth, Key])` + `toTaggedUnion("type")`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Value {
    #[serde(rename = "oauth")]
    OAuth {
        methodID: crate::integration_id::IntegrationMethodID,
        refresh: String,
        access: String,
        expires: crate::schema_primitives::NonNegativeInt,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<BTreeMap<String, serde_json::Value>>,
    },
    #[serde(rename = "key")]
    Key {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<BTreeMap<String, serde_json::Value>>,
    },
}
