#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/protocols/utils/openai-options.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_EFFECT_0: &str = "effect";
pub const VERBATIM_MAX_1: &str = "max";
pub const VERBATIM_FILE_SEARCH_CALL_RESULTS_2: &str = "file_search_call.results";
pub const VERBATIM_WEB_SEARCH_CALL_RESULTS_3: &str = "web_search_call.results";
pub const VERBATIM_WEB_SEARCH_CALL_ACTION_S_4: &str = "web_search_call.action.sources";
pub const VERBATIM_MESSAGE_INPUT_IMAGE_IMAG_5: &str = "message.input_image.image_url";
pub const VERBATIM_COMPUTER_CALL_OUTPUT_OUT_6: &str = "computer_call_output.output.image_url";
pub const VERBATIM_CODE_INTERPRETER_CALL_OU_7: &str = "code_interpreter_call.outputs";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "openai_options".to_string(),
        value: Value::Null,
    }
}
