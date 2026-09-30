//! Rust port of `packages/core/src/config/provider.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

// PROVISIONAL pending ProviderV2 + ModelV2

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cache {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<serde_json::Value>,
    pub input: f64,
    pub output: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Cache>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<std::collections::BTreeMap<String, Model>>,
}
