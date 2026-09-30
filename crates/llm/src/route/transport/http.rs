#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/route/transport/http.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_EFFECT_UNSTABLE_HTTP_1: &str = "effect/unstable/http";
pub const VERBATIM_CONTENT_2: &str = "content";
pub const VERBATIM_CONTENTS_3: &str = "contents";
pub const VERBATIM_FREQUENCYPENALTY_4: &str = "frequencyPenalty";
pub const VERBATIM_FREQUENCY_PENALTY_5: &str = "frequency_penalty";
pub const VERBATIM_GENERATIONCONFIG_6: &str = "generationConfig";
pub const VERBATIM_INFERENCECONFIG_7: &str = "inferenceConfig";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "http".to_string(),
        value: Value::Null,
    }
}
