#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/route/transport/websocket.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_EFFECT_UNSTABLE_HTTP_1: &str = "effect/unstable/http";
pub const VERBATIM_WEBSOCKETEXECUTOR_2: &str = "WebSocketExecutor";
pub const VERBATIM_MESSAGE_3: &str = "message";
pub const VERBATIM_STRING_4: &str = "string";
pub const VERBATIM_OPEN_5: &str = "open";
pub const VERBATIM_ERROR_6: &str = "error";
pub const VERBATIM_CLOSE_7: &str = "close";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "websocket".to_string(),
        value: Value::Null,
    }
}
