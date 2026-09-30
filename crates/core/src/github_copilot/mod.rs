//! Rust port of `packages/core/src/github-copilot` barrel + root items.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! (Merged: github_copilot.rs folded here — E0761 single-module rule; items byte-identical, only relocated.)

pub mod chat;
pub mod copilot_provider;
pub mod openai_compatible_error;
pub mod responses;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotInfo {
    pub token: String,
}

pub fn is_copilot_provider(id: &str) -> bool {
    id == "github-copilot"
}

// PROVISIONAL pending OAuth + provider wiring.
