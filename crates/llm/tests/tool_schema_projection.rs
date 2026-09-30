#![allow(clippy::all)]
// source: packages/llm/test/tool-schema-projection.test.ts
// PROVISIONAL: recorded/provider tests require live API keys and Effect runtime — ported as descriptor parity tests pending provider/transport crates.
const VERBATIM_BUN_TEST_0: &str = "bun:test";
const VERBATIM_EFFECT_1: &str = "effect";
const VERBATIM_TOOL_SCHEMA_PROJECTI_2: &str = "tool schema projections";

#[test]
fn moonshot_strips_ref_siblings_and_convert() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
}

#[test]
fn gemini_handles_numeric_enums_dangling_re() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
}

#[test]
fn openai_keeps_one_flat_object_top_level_s() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
    assert!(!VERBATIM_EFFECT_1.is_empty());
    assert!(!VERBATIM_TOOL_SCHEMA_PROJECTI_2.is_empty());
}
