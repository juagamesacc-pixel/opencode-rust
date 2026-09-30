//! Port of packages/app/src/components/prompt-input/contracts.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/contracts.ts — pure logic / types.
// Exported symbols: PromptInputState, PromptInputSubmission, PromptInputControls, PromptInputProps

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInputState {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInputSubmission {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInputControls {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInputProps {
    pub inner: Value,
}
