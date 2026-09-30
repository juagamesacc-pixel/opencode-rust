//! Port of `packages/schema/src/question.ts` (Question v2 contracts).
//!
//! Source exports: `ID` (brand `QuestionV2.ID`, loose `que` prefix),
//! `Option`, `Info`, `Prompt`, `Tool`, `Request`, `Answer`, `Reply`,
//! `Event.{Asked,Replied,Rejected,Definitions}`.
//!
//! Cross-lane: `SessionID` is `crate::session_id::SessionID` (Lane A), IDs use
//! `crate::identifier::ascending` (Lane A). `Event` markers carry verbatim
//! `type` strings for the assembly lane.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Branded `QuestionV2.ID` (loose `que` prefix check, canonical `que_…`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Canonical constructor (`que_` + ascending identifier).
    pub fn create() -> Self {
        Self(format!("que_{}", crate::identifier::ascending()))
    }

    /// Directional constructor preserved from source statics: wraps the given
    /// ID, or creates one when absent.
    pub fn ascending(id: std::option::Option<&str>) -> Self {
        match id {
            std::option::Option::Some(value) => Self(value.to_string()),
            std::option::Option::None => Self::create(),
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
                "QuestionV2.ID must start with \"que\": {value}"
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

/// `QuestionV2.Option`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Option {
    pub label: String,
    pub description: String,
}

/// `QuestionV2.Info` (shared `base` fields + `custom`).
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

/// `QuestionV2.Prompt` (shared `base` fields only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub question: String,
    pub header: String,
    pub options: Vec<Option>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub multiple: std::option::Option<bool>,
}

/// `QuestionV2.Tool`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    pub messageID: String,
    pub callID: String,
}

/// `QuestionV2.Request`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub id: ID,
    pub sessionID: crate::session_id::SessionID,
    pub questions: Vec<Info>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub tool: std::option::Option<Tool>,
}

/// `QuestionV2.Answer` (array of selected labels).
pub type Answer = Vec<String>;

/// `QuestionV2.Reply` (answers in order of questions).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reply {
    pub answers: Vec<Answer>,
}

/// Event definitions (`question.v2.asked`, `question.v2.replied`,
/// `question.v2.rejected`).
pub mod Event {
    /// `question.v2.asked` — payload [`super::Request`].
    pub struct Asked;
    impl Asked {
        pub const TYPE: &'static str = "question.v2.asked";
    }

    /// `question.v2.replied` — payload [`RepliedPayload`].
    pub struct Replied;
    impl Replied {
        pub const TYPE: &'static str = "question.v2.replied";
    }

    /// `question.v2.rejected` — payload [`RejectedPayload`].
    pub struct Rejected;
    impl Rejected {
        pub const TYPE: &'static str = "question.v2.rejected";
    }

    /// Payload of `question.v2.replied`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepliedPayload {
        pub sessionID: crate::session_id::SessionID,
        pub requestID: super::ID,
        pub answers: Vec<super::Answer>,
    }

    /// Payload of `question.v2.rejected`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RejectedPayload {
        pub sessionID: crate::session_id::SessionID,
        pub requestID: super::ID,
    }

    /// Verbatim declaration order: `Asked`, `Replied`, `Rejected`.
    pub const Definitions: &[&'static str] = &[Asked::TYPE, Replied::TYPE, Rejected::TYPE];
}
