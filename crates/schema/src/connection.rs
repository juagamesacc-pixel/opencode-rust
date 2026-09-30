//! Rust port of `packages/schema/src/connection.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! `Info` is `Union([CredentialInfo, EnvInfo]).pipe(Schema.toTaggedUnion("type"))` —
//! an internally tagged union on `type`; the enum carries inline variant fields
//! (the `type` tag key is supplied by serde), matching the key shape of the
//! exported member structs.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Identifier `Connection.CredentialInfo`. Field order is verbatim:
/// type, id, label.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialInfo {
    #[serde(rename = "type")]
    pub r#type: String,
    pub id: crate::credential::ID,
    pub label: String,
}

impl CredentialInfo {
    /// Verbatim literal of the `type` key.
    pub const TYPE: &'static str = "credential";
}

/// Identifier `Connection.EnvInfo`. Field order is verbatim: type, name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvInfo {
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
}

impl EnvInfo {
    /// Verbatim literal of the `type` key.
    pub const TYPE: &'static str = "env";
}

/// Identifier `Connection.Info` — `Union([CredentialInfo, EnvInfo])` + `toTaggedUnion("type")`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Info {
    #[serde(rename = "credential")]
    CredentialInfo {
        id: crate::credential::ID,
        label: String,
    },
    #[serde(rename = "env")]
    EnvInfo { name: String },
}
