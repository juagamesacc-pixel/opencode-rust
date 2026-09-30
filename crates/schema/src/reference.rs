//! Port of `packages/schema/src/reference.ts`.
//!
//! Source exports: `Event.{Updated,Definitions}`, `LocalSource`, `GitSource`,
//! `Source` (tagged union on `type`), `Info` (source `Schema.Class`).
//! Cross-lane: `AbsolutePath` is `crate::schema_primitives::AbsolutePath`
//! (Lane A). The union is untagged over member structs (each carries its own
//! verbatim `type` key); member order follows the source.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Event definitions (`reference.updated`, empty schema).
pub mod Event {
    /// `reference.updated`.
    pub struct Updated;
    impl Updated {
        pub const TYPE: &'static str = "reference.updated";
    }

    /// Verbatim declaration order: `Updated`.
    pub const Definitions: &[&'static str] = &[Updated::TYPE];
}

/// `Reference.LocalSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalSource {
    #[serde(rename = "type")]
    pub r#type: String,
    pub path: crate::schema_primitives::AbsolutePath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

impl LocalSource {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "local";
}

/// `Reference.GitSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitSource {
    #[serde(rename = "type")]
    pub r#type: String,
    pub repository: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

impl GitSource {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "git";
}

/// `Reference.Source` tagged union on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Source {
    Local(LocalSource),
    Git(GitSource),
}

/// `Reference.Info` (source `Schema.Class`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub path: crate::schema_primitives::AbsolutePath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    pub source: Source,
}
