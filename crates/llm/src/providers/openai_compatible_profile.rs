#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/openai-compatible-profile.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_BASETEN_0: &str = "baseten";
pub const VERBATIM_HTTPS_INFERENCE_BASETEN__1: &str = "https://inference.baseten.co/v1";
pub const VERBATIM_CEREBRAS_2: &str = "cerebras";
pub const VERBATIM_HTTPS_API_CEREBRAS_AI_V1_3: &str = "https://api.cerebras.ai/v1";
pub const VERBATIM_DEEPINFRA_4: &str = "deepinfra";
pub const VERBATIM_HTTPS_API_DEEPINFRA_COM__5: &str = "https://api.deepinfra.com/v1/openai";
pub const VERBATIM_DEEPSEEK_6: &str = "deepseek";
pub const VERBATIM_HTTPS_API_DEEPSEEK_COM_V_7: &str = "https://api.deepseek.com/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openai_compatible_profile".to_string(),
        value: Value::Null,
    }
}
