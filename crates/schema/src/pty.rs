//! Port of `packages/schema/src/pty.ts`.
//!
//! Source exports: `ID` (brand `PtyID`, loose `pty` prefix, with `create` +
//! `ascending`), `Info`, `Event.{Created,Updated,Exited,Deleted,Definitions}`,
//! `CreateInput`, `UpdateInput`. Cross-lane: `NonNegativeInt`/`PositiveInt` are
//! `crate::schema_primitives` (Lane A, `i64`), IDs use
//! `crate::identifier::ascending` (Lane A).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Branded `PtyID` (loose `pty` prefix check, canonical `pty_…`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Canonical constructor (`pty_` + ascending identifier).
    pub fn create() -> Self {
        Self(format!("pty_{}", crate::identifier::ascending()))
    }

    /// Directional constructor preserved from source statics.
    pub fn ascending(id: Option<&str>) -> Self {
        match id {
            Some(value) => Self(value.to_string()),
            None => Self::create(),
        }
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.starts_with("pty") {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "PtyID must start with \"pty\": {value}"
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

/// `Pty` `status` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "exited")]
    Exited,
}

/// `Pty.Info` (identifier `Pty`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    pub title: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub status: Status,
    pub pid: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exitCode: Option<crate::schema_primitives::NonNegativeInt>,
}

/// Event definitions (`pty.created`, `pty.updated`, `pty.exited`, `pty.deleted`).
pub mod Event {
    /// `pty.created` — payload [`CreatedPayload`].
    pub struct Created;
    impl Created {
        pub const TYPE: &'static str = "pty.created";
    }

    /// `pty.updated` — payload [`UpdatedPayload`].
    pub struct Updated;
    impl Updated {
        pub const TYPE: &'static str = "pty.updated";
    }

    /// `pty.exited` — payload [`ExitedPayload`].
    pub struct Exited;
    impl Exited {
        pub const TYPE: &'static str = "pty.exited";
    }

    /// `pty.deleted` — payload [`DeletedPayload`].
    pub struct Deleted;
    impl Deleted {
        pub const TYPE: &'static str = "pty.deleted";
    }

    /// Payload of `pty.created`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreatedPayload {
        pub info: super::Info,
    }

    /// Payload of `pty.updated`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UpdatedPayload {
        pub info: super::Info,
    }

    /// Payload of `pty.exited`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ExitedPayload {
        pub id: super::ID,
        pub exitCode: crate::schema_primitives::NonNegativeInt,
    }

    /// Payload of `pty.deleted`.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct DeletedPayload {
        pub id: super::ID,
    }

    /// Verbatim declaration order: `Created`, `Updated`, `Exited`, `Deleted`.
    pub const Definitions: &[&'static str] =
        &[Created::TYPE, Updated::TYPE, Exited::TYPE, Deleted::TYPE];
}

/// `Pty.CreateInput`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<BTreeMap<String, String>>,
}

/// `Pty.UpdateInput` size block (inline in source, kept in field order).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateInputSize {
    pub rows: crate::schema_primitives::PositiveInt,
    pub cols: crate::schema_primitives::PositiveInt,
}

/// `Pty.UpdateInput`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<UpdateInputSize>,
}
