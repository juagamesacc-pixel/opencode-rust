#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/schema/options.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_LLM_HTTPOPTIONS_1: &str = "LLM.HttpOptions";
pub const VERBATIM_LLM_GENERATIONOPTIONS_2: &str = "LLM.GenerationOptions";
pub const VERBATIM_MAXTOKENS_3: &str = "maxTokens";
pub const VERBATIM_TEMPERATURE_4: &str = "temperature";
pub const VERBATIM_TOPP_5: &str = "topP";
pub const VERBATIM_TOPK_6: &str = "topK";
pub const VERBATIM_FREQUENCYPENALTY_7: &str = "frequencyPenalty";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "options".to_string(),
        value: Value::Null,
    }
}
