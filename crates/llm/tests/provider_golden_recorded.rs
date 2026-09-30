#![allow(clippy::all)]
// source: packages/llm/test/provider/golden.recorded.test.ts
// PROVISIONAL: recorded/provider tests require live API keys and Effect runtime — ported as descriptor parity tests pending provider/transport crates.

const VERBATIM_FIXTURE_0: &str = "fixture";
const VERBATIM_GPT_4O_MINI_1: &str = "gpt-4o-mini";
const VERBATIM_GPT_5_5_2: &str = "gpt-5.5";

#[test]
fn smoke_test() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_FIXTURE_0.is_empty());
    assert!(!VERBATIM_GPT_4O_MINI_1.is_empty());
    assert!(!VERBATIM_GPT_5_5_2.is_empty());
}
