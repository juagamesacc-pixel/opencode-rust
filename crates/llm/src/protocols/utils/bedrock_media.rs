#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/utils/bedrock-media.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_PNG_1: &str = "png";
pub const VERBATIM_JPEG_2: &str = "jpeg";
pub const VERBATIM_GIF_3: &str = "gif";
pub const VERBATIM_WEBP_4: &str = "webp";
pub const VERBATIM_PDF_5: &str = "pdf";
pub const VERBATIM_CSV_6: &str = "csv";
pub const VERBATIM_DOC_7: &str = "doc";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "bedrock_media".to_string(),
        value: Value::Null,
    }
}
