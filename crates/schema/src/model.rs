//! Rust port of `packages/schema/src/model.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Brand `ModelV2.ID` — transparent newtype, wire format stays bare string.
///
/// No refinement check in source; any string is accepted.
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

/// Brand `VariantID` — transparent newtype, wire format stays bare string.
///
/// No refinement check in source; any string is accepted.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VariantID(pub String);

impl VariantID {
    pub fn new(value: String) -> Self {
        VariantID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::ops::Deref for VariantID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for VariantID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VariantID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for VariantID {
    fn from(value: String) -> Self {
        VariantID(value)
    }
}

impl From<&str> for VariantID {
    fn from(value: &str) -> Self {
        VariantID(value.to_string())
    }
}

/// Identifier `Model.Ref`. Source order: id, providerID, variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ref {
    pub id: ID,
    pub providerID: crate::provider::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<VariantID>,
}

/// Brand `Family` — transparent newtype, wire format stays bare string.
///
/// No refinement check in source; any string is accepted.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Family(pub String);

impl Family {
    pub fn new(value: String) -> Self {
        Family(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::ops::Deref for Family {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Family {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Family {
    fn from(value: String) -> Self {
        Family(value)
    }
}

impl From<&str> for Family {
    fn from(value: &str) -> Self {
        Family(value.to_string())
    }
}

/// Identifier `Model.Capabilities`. Source order: tools, input, output.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Capabilities {
    pub tools: bool,
    pub input: Vec<String>,
    pub output: Vec<String>,
}

/// Identifier `Model.Cost`. Source order: tier, input, output, cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<CostTier>,
    pub input: f64,
    pub output: f64,
    pub cache: CostCache,
}

/// `Model.Cost.tier` sub-struct — `Schema.Struct({ type: Literal("context"), size: Schema.Int })`
/// wrapped in `optional(...)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostTier {
    pub r#type: String,
    pub size: i64,
}

/// `Model.Cost.cache` sub-struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostCache {
    pub read: f64,
    pub write: f64,
}

/// Identifier `Model.Api` — `Union([ AISDK-like, Native-like ])` + `toTaggedUnion("type")`.
///
/// Source spreads `Provider.AISDK.fields` / `Provider.Native.fields` into
/// anonymous structs that also carry `id: ID`. The two union branches are
/// tagged on `type` (`"aisdk"` or `"native"`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Api {
    #[serde(rename = "aisdk")]
    AISDK {
        id: ID,
        package: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        settings: Option<BTreeMap<String, serde_json::Value>>,
    },
    #[serde(rename = "native")]
    Native {
        id: ID,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        settings: BTreeMap<String, serde_json::Value>,
    },
}

/// Identifier `ModelV2.Info`.
///
/// Source order: id, providerID, family, name, api, capabilities, request,
/// variants, time, cost, status, enabled, limit.
///
/// The inline `request` sub-struct spreads `Provider.Request.fields` and adds
/// `variant: optional(String)`. The `variants` array element also spreads
/// `Provider.Request.fields` and adds `id: VariantID`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    pub providerID: crate::provider::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
    pub name: String,
    pub api: Api,
    pub capabilities: Capabilities,
    pub request: InfoRequest,
    pub variants: Vec<InfoVariant>,
    pub time: InfoTime,
    pub cost: Vec<Cost>,
    pub status: String,
    pub enabled: bool,
    pub limit: InfoLimit,
}

/// Anonymous `request` sub-struct of `Info` — `Provider.Request` fields
/// (`headers`, `body`) plus `variant: optional(String)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoRequest {
    pub headers: BTreeMap<String, String>,
    pub body: BTreeMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

/// Anonymous `variants` array element — `Provider.Request` fields
/// (`headers`, `body`) plus `id: VariantID`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoVariant {
    pub id: VariantID,
    pub headers: BTreeMap<String, String>,
    pub body: BTreeMap<String, serde_json::Value>,
}

/// Anonymous `time` sub-struct of `Info`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoTime {
    pub released: f64,
}

/// Anonymous `limit` sub-struct of `Info`. Source order: context, input, output.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoLimit {
    pub context: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<i64>,
    pub output: i64,
}

impl Info {
    /// Port of `statics((schema) => ({ empty: (providerID, modelID) => ... }))`.
    ///
    /// Source:
    /// ```text
    /// schema.make({
    ///   id: modelID, providerID, name: modelID,
    ///   api: { id: modelID, type: "native", settings: {} },
    ///   capabilities: { tools: false, input: [], output: [] },
    ///   request: { headers: {}, body: {} },
    ///   variants: [], time: { released: 0 }, cost: [],
    ///   status: "active", enabled: true,
    ///   limit: { context: 0, output: 0 },
    /// })
    /// ```
    pub fn empty(providerID: crate::provider::ID, modelID: ID) -> Self {
        Self {
            id: modelID.clone(),
            providerID,
            family: None,
            name: modelID.0.clone(),
            api: Api::Native {
                id: modelID,
                url: None,
                settings: BTreeMap::new(),
            },
            capabilities: Capabilities {
                tools: false,
                input: Vec::new(),
                output: Vec::new(),
            },
            request: InfoRequest {
                headers: BTreeMap::new(),
                body: BTreeMap::new(),
                variant: None,
            },
            variants: Vec::new(),
            time: InfoTime { released: 0.0 },
            cost: Vec::new(),
            status: "active".to_owned(),
            enabled: true,
            limit: InfoLimit {
                context: 0,
                input: None,
                output: 0,
            },
        }
    }
}
