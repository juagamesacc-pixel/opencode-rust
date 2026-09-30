//! Rust port of `packages/core/src/oauth/*` + `packages/core/src/oauth` root items.
//! Source pin: v1.18.30 @3104c14.
//! (Merged: oauth.rs folded here — E0761 single-module rule; items byte-identical, only relocated.)

pub mod page;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

// PROVISIONAL pending OAuth flow wiring.
