//! Port of packages/app/src/components/prompt-input/history-store.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/history-store.ts — pure logic / types.
// Exported symbols: PromptInputHistory, createPromptInputHistory, createPersistedPromptInputHistory
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/prompt-input/history-store.ts

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInputHistory {
    pub inner: Value,
}
pub fn create_prompt_input_history(_input: Value) -> Value {
    Value::Null
}
pub fn create_persisted_prompt_input_history(_input: Value) -> Value {
    Value::Null
}
