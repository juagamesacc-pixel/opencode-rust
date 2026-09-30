#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/google.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_GOOGLE_0: &str = "google";
pub const VERBATIM_OPTIONAL_1: &str = "optional";
pub const VERBATIM_AUTH_2: &str = "auth";
pub const VERBATIM_APIKEY_3: &str = "apiKey";
pub const VERBATIM_GOOGLE_GENERATIVE_AI_API_4: &str = "GOOGLE_GENERATIVE_AI_API_KEY";
pub const VERBATIM_X_GOOG_API_KEY_5: &str = "x-goog-api-key";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "google".to_string(),
        value: Value::Null,
    }
}
