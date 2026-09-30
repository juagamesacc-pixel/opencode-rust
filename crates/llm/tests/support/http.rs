#![allow(clippy::all, dead_code)]
// source: packages/llm/packages/llm/test/lib/http.ts
// PROVISIONAL: test helper — descriptor pending effect/http runtime
use serde_json::Value;
#[derive(Debug, Clone)]
pub struct Helper {
    pub value: Value,
}
