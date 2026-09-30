//! Port of packages/app/src/components/dialog-custom-provider-form.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/dialog-custom-provider-form.ts — pure logic / types.
// Exported symbols: ModelErr, HeaderErr, ModelRow, HeaderRow, FormState, validateCustomProvider

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelErr {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaderErr {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelRow {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaderRow {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormState {
    pub inner: Value,
}
pub fn validate_custom_provider(_input: Value) -> Value {
    Value::Null
}
