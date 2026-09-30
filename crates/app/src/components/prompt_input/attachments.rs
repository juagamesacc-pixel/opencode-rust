//! Port of packages/app/src/components/prompt-input/attachments.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/attachments.ts — pure logic / types.
// Exported symbols: PromptAttachmentsInput, createPromptAttachmentsCore
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/prompt-input/attachments.ts

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptAttachmentsInput {
    pub inner: Value,
}
pub fn create_prompt_attachments_core(_input: Value) -> Value {
    Value::Null
}
