//! Port of `packages/schema/src/v1/permission.ts` (`PermissionV1`).
//!
//! Source exports: `ID` (brand `PermissionID`, loose `per` prefix), `Action`,
//! `Rule`, `Ruleset`, `Request`, `Reply`, `ReplyBody`, `Approval`, `AskInput`,
//! `ReplyInput`, `Event.{Asked,Replied,Definitions}`.
//!
//! Distinct from v2 `crate::permission` (no unification): field shapes differ
//! (`permission`/`patterns`/`metadata`/`always` vs v2 `action`/`resources`).
//! Cross-lane: `SessionID` (`crate::session_id`), `Project.ID`
//! (`crate::project`, Lane B), `crate::identifier::ascending` (Lane A).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Branded `PermissionID` (loose `per` prefix check, canonical `per_…`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Directional constructor preserved from source statics.
    pub fn ascending(id: Option<&str>) -> Self {
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
                "PermissionID must start with \"per\": {value}"
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

/// `PermissionAction` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "ask")]
    Ask,
}

/// `PermissionRule`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub permission: String,
    pub pattern: String,
    pub action: Action,
}

/// `PermissionRuleset`.
pub type Ruleset = Vec<Rule>;

/// `PermissionV1.Tool` inline shape (`{messageID, callID}`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestTool {
    pub messageID: String,
    pub callID: String,
}

/// `PermissionRequest`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub id: ID,
    pub sessionID: crate::session_id::SessionID,
    pub permission: String,
    pub patterns: Vec<String>,
    pub metadata: BTreeMap<String, serde_json::Value>,
    pub always: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<RequestTool>,
}

/// `PermissionV1.Reply` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Reply {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "reject")]
    Reject,
}

/// `PermissionReplyBody`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplyBody {
    pub reply: Reply,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `PermissionApproval`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Approval {
    pub projectID: crate::project::ID,
    pub patterns: Vec<String>,
}

/// `PermissionAskInput` (`Request` fields with optional `id`, plus `ruleset`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AskInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub sessionID: crate::session_id::SessionID,
    pub permission: String,
    pub patterns: Vec<String>,
    pub metadata: BTreeMap<String, serde_json::Value>,
    pub always: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<RequestTool>,
    pub ruleset: Ruleset,
}

/// `PermissionReplyInput` (`requestID` + `ReplyBody` fields).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplyInput {
    pub requestID: ID,
    pub reply: Reply,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Event definitions (`permission.asked`, `permission.replied`).
pub mod Event {
    /// `permission.asked` — payload [`super::Request`].
    pub struct Asked;
    impl Asked {
        pub const TYPE: &'static str = "permission.asked";
    }

    /// `permission.replied` — payload [`RepliedPayload`].
    pub struct Replied;
    impl Replied {
        pub const TYPE: &'static str = "permission.replied";
    }

    /// Payload of `permission.replied`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepliedPayload {
        pub sessionID: crate::session_id::SessionID,
        pub requestID: super::ID,
        pub reply: super::Reply,
    }

    /// Verbatim declaration order: `Asked`, `Replied`.
    pub const Definitions: &[&'static str] = &[Asked::TYPE, Replied::TYPE];
}
