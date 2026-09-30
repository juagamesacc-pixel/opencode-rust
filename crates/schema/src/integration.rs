//! Port of `packages/schema/src/integration.ts`.
//!
//! Source exports: `ID` (= `IntegrationID`), `MethodID` (=
//! `IntegrationMethodID`), `When`, `TextPrompt`, `SelectPrompt`, `Prompt`
//! (tagged union), `OAuthMethod`, `KeyMethod`, `EnvMethod`, `Method` (tagged
//! union), `Inputs`, `Event.{Updated,ConnectionUpdated,Definitions}`, `Ref`,
//! `Info`, `AttemptID`, `Attempt`, `AttemptStatus` (tagged union on `status`).
//!
//! Cross-lane per plan §4: `crate::integration_id::{IntegrationID,
//! IntegrationMethodID}` and `crate::connection::Info` (Lane A),
//! `crate::identifier::ascending` (Lane A). Unions are untagged over member
//! structs (each carries its own verbatim discriminator key); member order
//! follows the source.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// `Integration.ID` (= `IntegrationID`).
pub type ID = crate::integration_id::IntegrationID;

/// `Integration.MethodID` (= `IntegrationMethodID`).
pub type MethodID = crate::integration_id::IntegrationMethodID;

/// `Integration.When`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct When {
    pub key: String,
    pub op: WhenOp,
    pub value: String,
}

/// `Integration.When` `op` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WhenOp {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    Neq,
}

/// `Integration.TextPrompt`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextPrompt {
    #[serde(rename = "type")]
    pub r#type: String,
    pub key: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<When>,
}

impl TextPrompt {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "text";
}

/// `Integration.SelectPrompt` option entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectPromptOption {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// `Integration.SelectPrompt`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectPrompt {
    #[serde(rename = "type")]
    pub r#type: String,
    pub key: String,
    pub message: String,
    pub options: Vec<SelectPromptOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<When>,
}

impl SelectPrompt {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "select";
}

/// `Integration.Prompt` tagged union on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Prompt {
    Text(TextPrompt),
    Select(SelectPrompt),
}

/// `Integration.OAuthMethod`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthMethod {
    pub id: MethodID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<Prompt>>,
}

impl OAuthMethod {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "oauth";
}

/// `Integration.KeyMethod`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyMethod {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl KeyMethod {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "key";
}

/// `Integration.EnvMethod`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvMethod {
    #[serde(rename = "type")]
    pub r#type: String,
    pub names: Vec<String>,
}

impl EnvMethod {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "env";
}

/// `Integration.Method` tagged union on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Method {
    OAuth(OAuthMethod),
    Key(KeyMethod),
    Env(EnvMethod),
}

/// `Integration.Inputs` (string record).
pub type Inputs = BTreeMap<String, String>;

/// Event definitions (`integration.updated`, `integration.connection.updated`).
pub mod Event {
    /// `integration.updated` (empty schema).
    pub struct Updated;
    impl Updated {
        pub const TYPE: &'static str = "integration.updated";
    }

    /// `integration.connection.updated` — payload [`ConnectionUpdatedPayload`].
    pub struct ConnectionUpdated;
    impl ConnectionUpdated {
        pub const TYPE: &'static str = "integration.connection.updated";
    }

    /// Payload of `integration.connection.updated`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ConnectionUpdatedPayload {
        pub integrationID: super::ID,
    }

    /// Verbatim declaration order: `Updated`, `ConnectionUpdated`.
    pub const Definitions: &[&'static str] = &[Updated::TYPE, ConnectionUpdated::TYPE];
}

/// `Integration.Ref`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ref {
    pub id: ID,
    pub name: String,
}

/// `Integration.Info` (source `Schema.Class`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    pub name: String,
    pub methods: Vec<Method>,
    pub connections: Vec<crate::connection::Info>,
}

/// Branded `Integration.AttemptID` (no prefix check in source).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AttemptID(pub String);

impl AttemptID {
    /// Canonical constructor (`con_` + ascending identifier).
    pub fn create() -> Self {
        Self(format!("con_{}", crate::identifier::ascending()))
    }
}

impl std::fmt::Display for AttemptID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for AttemptID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// `Integration.Attempt` time block (source-private `AttemptTime`, kept in
/// field order).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttemptTime {
    pub created: f64,
    pub expires: f64,
}

/// `Integration.Attempt` (source `Schema.Class`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attempt {
    pub attemptID: AttemptID,
    pub url: String,
    pub instructions: String,
    pub mode: AttemptMode,
    pub time: AttemptTime,
}

/// `Integration.Attempt` `mode` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AttemptMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "code")]
    Code,
}

/// `Integration.AttemptStatus` pending variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttemptStatusPending {
    pub status: String,
    pub time: AttemptTime,
}

/// `Integration.AttemptStatus` complete variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttemptStatusComplete {
    pub status: String,
    pub time: AttemptTime,
}

/// `Integration.AttemptStatus` failed variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttemptStatusFailed {
    pub status: String,
    pub message: String,
    pub time: AttemptTime,
}

/// `Integration.AttemptStatus` expired variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttemptStatusExpired {
    pub status: String,
    pub time: AttemptTime,
}

/// `Integration.AttemptStatus` tagged union on `status` (source order:
/// pending, complete, failed, expired).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttemptStatus {
    Pending(AttemptStatusPending),
    Complete(AttemptStatusComplete),
    Failed(AttemptStatusFailed),
    Expired(AttemptStatusExpired),
}
