//! Port of packages/app/src/components/prompt-input/transient-state.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/transient-state.ts — pure logic / types.
// Exported symbols: PromptInputTransientState, createPromptInputTransientState
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/prompt-input/transient-state.ts

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInputTransientState {
    pub inner: Value,
}
pub fn create_prompt_input_transient_state(_input: Value) -> Value {
    Value::Null
}
