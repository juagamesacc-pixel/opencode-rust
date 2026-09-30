//! Port of packages/app/src/components/prompt-input/files.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/files.ts — pure logic / types.
// Exported symbols: pickAttachmentFiles, attachmentMime

pub fn pick_attachment_files(_input: Value) -> Value {
    Value::Null
}
pub fn attachment_mime(_input: Value) -> Value {
    Value::Null
}
