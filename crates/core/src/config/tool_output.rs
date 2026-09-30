//! Rust port of `packages/core/src/config/tool-output.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<u32>,
}
