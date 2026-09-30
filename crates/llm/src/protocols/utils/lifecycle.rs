#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/utils/lifecycle.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "lifecycle".to_string(),
        value: Value::Null,
    }
}
