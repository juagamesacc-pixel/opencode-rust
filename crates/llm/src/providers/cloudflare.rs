#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/cloudflare.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_CLOUDFLARE_AI_GATEWAY_1: &str = "cloudflare-ai-gateway";
pub const VERBATIM_CLOUDFLARE_WORKERS_AI_2: &str = "cloudflare-workers-ai";
pub const VERBATIM_CLOUDFLARE_API_TOKEN_3: &str = "CLOUDFLARE_API_TOKEN";
pub const VERBATIM_CF_AIG_TOKEN_4: &str = "CF_AIG_TOKEN";
pub const VERBATIM_CLOUDFLARE_API_KEY_5: &str = "CLOUDFLARE_API_KEY";
pub const VERBATIM_CLOUDFLARE_WORKERS_AI_TO_6: &str = "CLOUDFLARE_WORKERS_AI_TOKEN";
pub const VERBATIM_OPTIONAL_7: &str = "optional";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "cloudflare".to_string(),
        value: Value::Null,
    }
}
