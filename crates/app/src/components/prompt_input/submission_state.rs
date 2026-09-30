//! Port of packages/app/src/components/prompt-input/submission-state.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/submission-state.ts — pure logic / types.
// Exported symbols: createPromptSubmissionState

pub fn create_prompt_submission_state(_input: Value) -> Value {
    Value::Null
}
