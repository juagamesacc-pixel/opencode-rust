//! Port of `packages/schema/src/skill.ts`.
//!
//! Source exports: `DirectorySource`, `UrlSource`, `Info`, `EmbeddedSource`,
//! `Source` (tagged union on `type` with `equals` + `key` helpers).
//! Cross-lane: `AbsolutePath` is `crate::schema_primitives::AbsolutePath`
//! (Lane A). The union is untagged over member structs (each carries its own
//! verbatim `type` key); member order follows the source. `EmbeddedSource`
//! boxes its recursive `skill` field (ports `Schema.suspend`).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `SkillV2.DirectorySource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectorySource {
    #[serde(rename = "type")]
    pub r#type: String,
    pub path: crate::schema_primitives::AbsolutePath,
}

impl DirectorySource {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "directory";
}

/// `SkillV2.UrlSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlSource {
    #[serde(rename = "type")]
    pub r#type: String,
    pub url: String,
}

impl UrlSource {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "url";
}

/// `SkillV2.Info`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slash: Option<bool>,
    pub location: crate::schema_primitives::AbsolutePath,
    pub content: String,
}

/// `SkillV2.EmbeddedSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedSource {
    #[serde(rename = "type")]
    pub r#type: String,
    pub skill: Box<Info>,
}

impl EmbeddedSource {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "embedded";
}

/// `SkillV2.Source` tagged union on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Source {
    Directory(DirectorySource),
    Url(UrlSource),
    Embedded(EmbeddedSource),
}

impl Source {
    /// Structural equality from source (`Source.equals`).
    pub fn equals(a: &Source, b: &Source) -> bool {
        match (a, b) {
            (Source::Directory(a), Source::Directory(b)) => a.path == b.path,
            (Source::Url(a), Source::Url(b)) => a.url == b.url,
            (Source::Embedded(a), Source::Embedded(b)) => a.skill.name == b.skill.name,
            _ => false,
        }
    }

    /// Stable key from source (`Source.key`).
    pub fn key(source: &Source) -> String {
        match source {
            Source::Directory(inner) => format!("directory:{}", inner.path.0),
            Source::Url(inner) => format!("url:{}", inner.url),
            Source::Embedded(inner) => format!("embedded:{}", inner.skill.name),
        }
    }
}
