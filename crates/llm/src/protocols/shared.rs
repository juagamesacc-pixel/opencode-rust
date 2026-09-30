#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/shared.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_NODE_BUFFER_0: &str = "node:buffer";
pub const VERBATIM_EFFECT_1: &str = "effect";
pub const VERBATIM_EFFECT_UNSTABLE_ENCODING_2: &str = "effect/unstable/encoding/Sse";
pub const VERBATIM_EFFECT_UNSTABLE_HTTP_3: &str = "effect/unstable/http";
pub const VERBATIM_PROVIDERSHARED_4: &str = "ProviderShared";
pub const VERBATIM_STREAM_5: &str = "stream";
pub const VERBATIM__CONST_ESCAPESYSTEMUPDAT_6: &str = ")

const escapeSystemUpdateText = (text: string) =>
  text.replaceAll(";
pub const VERBATIM__AMP__7: &str = "&amp;";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "shared".to_string(),
        value: Value::Null,
    }
}
