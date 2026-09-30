//! Port of `packages/schema/src/permission.ts` (Permission v2 contracts).
//!
//! Source exports: `ID` (brand `PermissionV2.ID`, loose `per` prefix),
//! `Source`, `Request`, `Reply`, `Event.{Asked,Replied,Definitions}`,
//! `Effect`, `Rule`, `Ruleset`.
//!
//! Cross-lane: `SessionID` is `crate::session_id::SessionID` (Lane A) and ID
//! creation uses `crate::identifier::ascending` (Lane A) per plan §4.
//! `Event` marker structs carry the verbatim `type` strings; the assembly lane
//! wires them to `crate::event` descriptors. Payload field order and JSON keys
//! follow the source exactly.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Branded `PermissionV2.ID`.
///
/// Decode accepts any `per`-prefixed string (matches the source
/// `Schema.isStartsWith("per")` looseness); `create` emits canonical `per_…`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Canonical constructor (`per_` + ascending identifier, or the given ID).
    pub fn create(id: Option<&str>) -> Self {
        match id {
            Some(value) => Self(value.to_string()),
            None => Self(format!("per_{}", crate::identifier::ascending())),
        }
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.starts_with("per") {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "PermissionV2.ID must start with \"per\": {value}"
            )))
        }
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// `PermissionV2.Source` — tagged union (single `tool` variant in source).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Source {
    #[serde(rename = "tool")]
    Tool { messageID: String, callID: String },
}

/// `PermissionV2.Request` (`id` + `RequestFields` in source order).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub id: ID,
    pub sessionID: crate::session_id::SessionID,
    pub action: String,
    pub resources: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

/// `PermissionV2.Reply` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Reply {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "reject")]
    Reject,
}

/// `PermissionV2.Effect` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "ask")]
    Ask,
}

/// `PermissionV2.Rule`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub action: String,
    pub resource: String,
    pub effect: Effect,
}

/// `PermissionV2.Ruleset` (array of `Rule`).
pub type Ruleset = Vec<Rule>;

/// Event definitions (`permission.v2.asked`, `permission.v2.replied`).
///
/// Marker structs preserve the `Event.Asked` / `Event.Replied` paths and the
/// verbatim `type` strings; payloads are [`Request`] and the inline replied
/// `{sessionID, requestID, reply}` shape (`RepliedPayload`).
pub mod Event {
    /// `permission.v2.asked` — payload [`super::Request`].
    pub struct Asked;
    impl Asked {
        pub const TYPE: &'static str = "permission.v2.asked";
    }

    /// `permission.v2.replied` — payload [`RepliedPayload`].
    pub struct Replied;
    impl Replied {
        pub const TYPE: &'static str = "permission.v2.replied";
    }

    /// Payload of `permission.v2.replied`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepliedPayload {
        pub sessionID: crate::session_id::SessionID,
        pub requestID: super::ID,
        pub reply: super::Reply,
    }

    /// Verbatim declaration order: `Asked`, `Replied`.
    pub const Definitions: &[&'static str] = &[Asked::TYPE, Replied::TYPE];
}
