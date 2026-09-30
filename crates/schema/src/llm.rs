//! Port of `packages/schema/src/llm.ts`.
//!
//! Source exports: `ProviderMetadata`, `ToolTextContent`, `ToolFileContent`,
//! `ToolContent` (tagged union on `type`). `Record` maps to `BTreeMap`,
//! `Unknown` maps to `serde_json::Value`. The union is untagged over member
//! structs (each carries its own `type` key, as in source); member order
//! follows the source (`ToolTextContent`, then `ToolFileContent`).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// `LLM.ProviderMetadata` (record of records of unknowns).
pub type ProviderMetadata = BTreeMap<String, BTreeMap<String, serde_json::Value>>;

/// `Tool.TextContent`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolTextContent {
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: String,
}

impl ToolTextContent {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "text";

    pub fn create(text: String) -> Self {
        Self {
            r#type: Self::TYPE.to_string(),
            text,
        }
    }
}

/// `Tool.FileContent`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolFileContent {
    #[serde(rename = "type")]
    pub r#type: String,
    pub uri: String,
    pub mime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ToolFileContent {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "file";

    pub fn create(uri: String, mime: String, name: Option<String>) -> Self {
        Self {
            r#type: Self::TYPE.to_string(),
            uri,
            mime,
            name,
        }
    }
}

/// `LLM.ToolContent` tagged union on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolContent {
    Text(ToolTextContent),
    File(ToolFileContent),
}
