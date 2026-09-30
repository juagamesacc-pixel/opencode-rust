#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/schema/events.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_FRESH_1: &str = "fresh";
pub const VERBATIM_LLM_USAGE_2: &str = "LLM.Usage";
pub const VERBATIM_STEP_START_3: &str = "step-start";
pub const VERBATIM_LLM_EVENT_STEPSTART_4: &str = "LLM.Event.StepStart";
pub const VERBATIM_TEXT_START_5: &str = "text-start";
pub const VERBATIM_LLM_EVENT_TEXTSTART_6: &str = "LLM.Event.TextStart";
pub const VERBATIM_TEXT_DELTA_7: &str = "text-delta";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "events".to_string(),
        value: Value::Null,
    }
}
