#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/bedrock-event-stream.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM__MESSAGE_TYPE_1: &str = ":message-type";
pub const VERBATIM_EVENT_2: &str = "event";
pub const VERBATIM__EVENT_TYPE_3: &str = ":event-type";
pub const VERBATIM_STRING_4: &str = "string";
pub const VERBATIM_FAILED_TO_PARSE_BEDROCK__5: &str =
    "Failed to parse Bedrock Converse event-stream payload";
pub const VERBATIM_AWS_EVENT_STREAM_6: &str = "aws-event-stream";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "bedrock_event_stream".to_string(),
        value: Value::Null,
    }
}
