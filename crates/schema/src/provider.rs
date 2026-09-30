//! Rust port of `packages/schema/src/provider.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Brand `ProviderV2.ID` — transparent newtype, wire format stays bare string.
///
/// No refinement check in source; any string is accepted.
/// `statics` provides named constants for well-known providers.
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

    // statics — verbatim keys from source.
    pub fn opencode() -> Self {
        Self("opencode".to_owned())
    }
    pub fn anthropic() -> Self {
        Self("anthropic".to_owned())
    }
    pub fn openai() -> Self {
        Self("openai".to_owned())
    }
    pub fn google() -> Self {
        Self("google".to_owned())
    }
    pub fn googleVertex() -> Self {
        Self("google-vertex".to_owned())
    }
    pub fn githubCopilot() -> Self {
        Self("github-copilot".to_owned())
    }
    pub fn amazonBedrock() -> Self {
        Self("amazon-bedrock".to_owned())
    }
    pub fn azure() -> Self {
        Self("azure".to_owned())
    }
    pub fn openrouter() -> Self {
        Self("openrouter".to_owned())
    }
    pub fn mistral() -> Self {
        Self("mistral".to_owned())
    }
    pub fn gitlab() -> Self {
        Self("gitlab".to_owned())
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

/// Identifier `Provider.AISDK`. Source order: type, package, url, settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AISDK {
    pub r#type: String,
    pub package: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<BTreeMap<String, serde_json::Value>>,
}

/// Identifier `Provider.Native`. Source order: type, url, settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Native {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub settings: BTreeMap<String, serde_json::Value>,
}

/// Identifier `Provider.Api` — `Union([AISDK, Native])` + `toTaggedUnion("type")`.
///
/// Enum variants carry inline fields (the `type` tag key is supplied by serde);
/// variant names match source member names verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Api {
    #[serde(rename = "aisdk")]
    AISDK {
        package: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        settings: Option<BTreeMap<String, serde_json::Value>>,
    },
    #[serde(rename = "native")]
    Native {
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        settings: BTreeMap<String, serde_json::Value>,
    },
}

/// Identifier `Provider.Request`. Source order: headers, body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub headers: BTreeMap<String, String>,
    pub body: BTreeMap<String, serde_json::Value>,
}

/// Identifier `ProviderV2.Info`. Source order: id, integrationID, name, disabled, api, request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrationID: Option<crate::integration::ID>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub api: Api,
    pub request: Request,
}

impl Info {
    /// Port of `statics((schema) => ({ empty: (id) => ... }))`.
    ///
    /// Source: `schema.make({ id, name: id, api: { type: "native", settings: {} }, request: { headers: {}, body: {} } })`.
    pub fn empty(id: ID) -> Self {
        Self {
            name: id.0.clone(),
            id,
            integrationID: None,
            disabled: None,
            api: Api::Native {
                url: None,
                settings: BTreeMap::new(),
            },
            request: Request {
                headers: BTreeMap::new(),
                body: BTreeMap::new(),
            },
        }
    }
}
