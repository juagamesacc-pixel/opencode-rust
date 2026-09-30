#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/utils/bedrock-auth.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_AWS4FETCH_0: &str = "aws4fetch";
pub const VERBATIM_EFFECT_1: &str = "effect";
pub const VERBATIM_EFFECT_UNSTABLE_HTTP_2: &str = "effect/unstable/http";
pub const VERBATIM_POST_3: &str = "POST";
pub const VERBATIM_BEDROCK_4: &str = "bedrock";
pub const VERBATIM_BEDROCK_CONVERSE_REQUIRE_5: &str =
    "Bedrock Converse requires either route bearer auth or AWS credentials configured on the route";
pub const VERBATIM_CONTENT_TYPE_6: &str = "content-type";
pub const VERBATIM_APPLICATION_JSON_7: &str = "application/json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "bedrock_auth".to_string(),
        value: Value::Null,
    }
}
