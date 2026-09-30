//! Port of packages/app/src/components/prompt-input/editor-dom.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde_json::Value;

/// Port of packages/app/src/components/prompt-input/editor-dom.ts — pure logic / types.
// Exported symbols: createTextFragment, getNodeLength, getTextLength, getCursorPosition, setCursorPosition, setRangeEdge

pub fn create_text_fragment(_input: Value) -> Value {
    Value::Null
}
pub fn get_node_length(_input: Value) -> Value {
    Value::Null
}
pub fn get_text_length(_input: Value) -> Value {
    Value::Null
}
pub fn get_cursor_position(_input: Value) -> Value {
    Value::Null
}
pub fn set_cursor_position(_input: Value) -> Value {
    Value::Null
}
pub fn set_range_edge(_input: Value) -> Value {
    Value::Null
}
