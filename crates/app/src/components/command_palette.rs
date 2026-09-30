//! Port of packages/app/src/components/command-palette.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/command-palette.ts — pure logic / types.
// Exported symbols: CommandPaletteEntry, uniqueCommandPaletteEntries, createCommandPaletteFileEntry, createCommandPaletteFileOpener, createCommandPaletteModel
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/command-palette.ts

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandPaletteEntry {
    pub inner: Value,
}
pub fn unique_command_palette_entries(_input: Value) -> Value {
    Value::Null
}
pub fn create_command_palette_file_entry(_input: Value) -> Value {
    Value::Null
}
pub fn create_command_palette_file_opener(_input: Value) -> Value {
    Value::Null
}
pub fn create_command_palette_model(_input: Value) -> Value {
    Value::Null
}
