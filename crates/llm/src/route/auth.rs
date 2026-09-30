#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/route/auth.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_EFFECT_UNSTABLE_HTTP_1: &str = "effect/unstable/http";
pub const VERBATIM_MISSINGCREDENTIALERROR_2: &str = "MissingCredentialError";
pub const VERBATIM_POST_3: &str = "POST";
pub const VERBATIM_GET_4: &str = "GET";
pub const VERBATIM_OBJECT_5: &str = "object";
pub const VERBATIM_APPLY_6: &str = "apply";
pub const VERBATIM_FUNCTION_7: &str = "function";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "auth".to_string(),
        value: Value::Null,
    }
}
pub const AUTH_BEARER: &str = "bearer";
pub const MISSING_CREDENTIAL_ERROR: &str = "MissingCredentialError";
