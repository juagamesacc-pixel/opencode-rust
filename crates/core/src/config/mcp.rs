//! Rust port of `packages/core/src/config/mcp.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Local {
    #[serde(rename = "type")]
    pub r#type: String,
    pub command: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timeout>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Remote {
    #[serde(rename = "type")]
    pub r#type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timeout>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Server {
    #[serde(rename = "local")]
    Local(Local),
    #[serde(rename = "remote")]
    Remote(Remote),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timeout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<std::collections::BTreeMap<String, Server>>,
}
