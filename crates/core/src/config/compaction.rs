//! Rust port of `packages/core/src/config/compaction.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keep {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep: Option<Keep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<u32>,
}
