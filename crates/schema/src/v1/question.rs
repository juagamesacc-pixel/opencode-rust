//! Port of `packages/schema/src/v1/question.ts` (`QuestionV1`).
//!
//! Source exports: `ID` (brand `QuestionID`, loose `que` prefix), `Option`,
//! `Info`, `Prompt`, `Tool` (with `SessionV1.MessageID`), `Request`,
//! `Answer`, `Reply`, `Replied`, `Rejected`,
//! `Event.{Asked,Replied,Rejected,Definitions}`.
//!
//! Distinct from v2 `crate::question` (different identifiers/brands; `Tool`
//! references the v1 message ID). Cross-lane: `SessionID`
//! (`crate::session_id`), `SessionV1.MessageID` (`super::session`, this lane).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Branded `QuestionID` (loose `que` prefix check, canonical `que_…`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Directional constructor preserved from source statics.
    pub fn ascending(id: std::option::Option<&str>) -> Self {
        match id {
            std::option::Option::Some(value) => Self(value.to_string()),
            std::option::Option::None => Self(format!("que_{}", crate::identifier::ascending())),
        }
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.starts_with("que") {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "QuestionID must start with \"que\": {value}"
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

/// `QuestionOption`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Option {
    pub label: String,
    pub description: String,
}

/// `QuestionInfo` (shared `base` fields + `custom`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub question: String,
    pub header: String,
    pub options: Vec<Option>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub multiple: std::option::Option<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub custom: std::option::Option<bool>,
}

/// `QuestionPrompt` (shared `base` fields only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub question: String,
    pub header: String,
    pub options: Vec<Option>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub multiple: std::option::Option<bool>,
}

/// `QuestionTool` (message ID is the v1 message brand).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    pub messageID: super::session::MessageID,
    pub callID: String,
}

/// `QuestionRequest`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub id: ID,
    pub sessionID: crate::session_id::SessionID,
    pub questions: Vec<Info>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub tool: std::option::Option<Tool>,
}

/// `QuestionAnswer`.
pub type Answer = Vec<String>;

/// `QuestionReply`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reply {
    pub answers: Vec<Answer>,
}

/// `QuestionReplied`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Replied {
    pub sessionID: crate::session_id::SessionID,
    pub requestID: ID,
    pub answers: Vec<Answer>,
}

/// `QuestionRejected`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rejected {
    pub sessionID: crate::session_id::SessionID,
    pub requestID: ID,
}

/// Event definitions (`question.asked`, `question.replied`, `question.rejected`).
pub mod Event {
    /// `question.asked` — payload [`super::Request`].
    pub struct Asked;
    impl Asked {
        pub const TYPE: &'static str = "question.asked";
    }

    /// `question.replied` — payload [`super::Replied`].
    pub struct Replied;
    impl Replied {
        pub const TYPE: &'static str = "question.replied";
    }

    /// `question.rejected` — payload [`super::Rejected`].
    pub struct Rejected;
    impl Rejected {
        pub const TYPE: &'static str = "question.rejected";
    }

    /// Verbatim declaration order: `Asked`, `Replied`, `Rejected`.
    pub const Definitions: &[&'static str] = &[Asked::TYPE, Replied::TYPE, Rejected::TYPE];
}
