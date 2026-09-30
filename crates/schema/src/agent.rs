//! Rust port of `packages/schema/src/agent.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Brand `AgentV2.ID` — transparent newtype, wire format stays bare string.
///
/// No refinement check in source (`Schema.String.pipe(Schema.brand("AgentV2.ID"))`).
/// Any string is accepted.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    pub fn new(value: String) -> Self {
        ID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::ops::Deref for ID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
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

/// Identifier `Agent.Color`.
///
/// Union of hex pattern (`/^#[0-9a-fA-F]{6}$/`) and named literals
/// (`primary | secondary | accent | success | warning | error | info`).
/// Wire format is a bare string; both branches serialize as the same string
/// shape. Runtime validation available via `is_hex()` and `is_named()`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Color(pub String);

impl Color {
    /// Runtime check for the hex-pattern branch (`#` + 6 hex digits).
    pub fn is_hex(&self) -> bool {
        let b = self.0.as_bytes();
        b.len() == 7 && b[0] == b'#' && b[1..].iter().all(|c| c.is_ascii_hexdigit())
    }

    /// Runtime check for the named-literal branch.
    pub fn is_named(&self) -> bool {
        matches!(
            self.0.as_str(),
            "primary" | "secondary" | "accent" | "success" | "warning" | "error" | "info"
        )
    }
}

impl std::ops::Deref for Color {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        Color(value)
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Color(value.to_string())
    }
}

/// Identifier `AgentV2.Info`.
///
/// Field order follows source order: `id`, `model`, `request`, `system`,
/// `description`, `mode`, `hidden`, `color`, `steps`, `permissions`.
/// `mode` is `subagent | primary | all` — inline literals kept as `String`
/// to avoid new pub names; wire shape is identical.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::model::Ref>,
    pub request: crate::provider::Request,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub mode: String,
    pub hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    pub permissions: crate::permission::Ruleset,
}

impl Info {
    /// Port of `statics((schema) => ({ empty: (id) => ... }))`.
    ///
    /// Source: `schema.make({ id, request: { headers: {}, body: {} }, mode: "all", hidden: false, permissions: [] })`.
    pub fn empty(id: ID) -> Self {
        Self {
            id,
            model: None,
            request: crate::provider::Request {
                headers: BTreeMap::new(),
                body: BTreeMap::new(),
            },
            system: None,
            description: None,
            mode: "all".to_owned(),
            hidden: false,
            color: None,
            steps: None,
            permissions: Vec::new(),
        }
    }
}
