//! Rust port of `packages/core/src/config/experimental.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

// PROVISIONAL pending Catalog + Policy crates

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Policy {
    // PROVISIONAL pending PolicyV2.Info.fields + PolicyAction union
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Experimental {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}
