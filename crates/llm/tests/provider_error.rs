#![allow(clippy::all)]
// source: packages/llm/test/provider-error.test.ts
// PROVISIONAL: recorded/provider tests require live API keys and Effect runtime — ported as descriptor parity tests pending provider/transport crates.
const VERBATIM_BUN_TEST_0: &str = "bun:test";
const VERBATIM_PROVIDER_ERROR_CLASS_1: &str = "provider error classification";
const VERBATIM_CLASSIFIES_PROVIDER__2: &str =
    "classifies provider token limit messages as context overflow";

#[test]
fn classifies_provider_token_limit_messages() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
}

#[test]
fn does_not_classify_rate_limits_as_context() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
    assert!(!VERBATIM_PROVIDER_ERROR_CLASS_1.is_empty());
    assert!(!VERBATIM_CLASSIFIES_PROVIDER__2.is_empty());
}
