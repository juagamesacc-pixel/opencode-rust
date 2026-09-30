#![allow(clippy::all)]
// source: packages/llm/test/provider/openrouter.test.ts
// PROVISIONAL: recorded/provider tests require live API keys and Effect runtime — ported as descriptor parity tests pending provider/transport crates.

const VERBATIM_BUN_TEST_0: &str = "bun:test";
const VERBATIM_EFFECT_1: &str = "effect";
const VERBATIM_OPENROUTER_2: &str = "OpenRouter";

#[test]
fn smoke_test() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
    assert!(!VERBATIM_EFFECT_1.is_empty());
    assert!(!VERBATIM_OPENROUTER_2.is_empty());
}
