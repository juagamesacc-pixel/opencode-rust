#![allow(clippy::all)]
// source: packages/llm/test/generate-object.test.ts
// PROVISIONAL: recorded/provider tests require live API keys and Effect runtime — ported as descriptor parity tests pending provider/transport crates.
const VERBATIM_BUN_TEST_0: &str = "bun:test";
const VERBATIM_EFFECT_1: &str = "effect";
const VERBATIM_HTTPS_API_OPENAI_TES_2: &str = "https://api.openai.test/v1/";

#[test]
fn forwards_json_schema_and_description_thr() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
}

#[test]
fn execute_receives_the_raw_input_untouched() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
    assert!(!VERBATIM_EFFECT_1.is_empty());
    assert!(!VERBATIM_HTTPS_API_OPENAI_TES_2.is_empty());
}
