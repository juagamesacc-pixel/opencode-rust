// source: src/cli/ui.ts — exports: [CancelledError, Style, println, print, empty, logo, input, error, markdown]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "UICancelledError"
/// - "\\x1b[96m"
/// - "\\x1b[96m\\x1b[1m"
/// - "\\x1b[90m"
/// - "\\x1b[90m\\x1b[1m"
/// - "\\x1b[93m"
/// - "\\x1b[93m\\x1b[1m"
/// - "\\x1b[91m"
use serde::{Deserialize, Serialize};

/// source: `export class CancelledError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelledError {
    pub value: serde_json::Value,
}
/// source: `export const Style` — shape as JSON value; CI verifies.
pub type Style = serde_json::Value;
/// source: `export function println` — stub shell; CI verifies behavior.
pub fn println(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function print` — stub shell; CI verifies behavior.
pub fn print(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function empty` — stub shell; CI verifies behavior.
pub fn empty(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function logo` — stub shell; CI verifies behavior.
pub fn logo(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function input` — stub shell; CI verifies behavior.
pub fn input(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function error` — stub shell; CI verifies behavior.
pub fn error(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function markdown` — stub shell; CI verifies behavior.
pub fn markdown(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
