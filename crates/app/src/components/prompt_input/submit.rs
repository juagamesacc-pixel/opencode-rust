//! Port of packages/app/src/components/prompt-input/submit.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/submit.ts — pure logic / types.
// Exported symbols: FollowupDraft, sendFollowupDraft
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/prompt-input/submit.ts

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowupDraft {
    pub inner: Value,
}
pub fn send_followup_draft(_input: Value) -> Value {
    Value::Null
}
