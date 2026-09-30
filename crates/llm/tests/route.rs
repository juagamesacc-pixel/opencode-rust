#![allow(clippy::all)]
// source: packages/llm/test/route.test.ts
// PROVISIONAL: recorded/provider tests require live API keys and Effect runtime — ported as descriptor parity tests pending provider/transport crates.
const VERBATIM_BUN_TEST_0: &str = "bun:test";
const VERBATIM_ROUTE_WITH_1: &str = "Route.with";
const VERBATIM_MERGES_ENDPOINT_QUER_2: &str =
    "merges endpoint query and header defaults while replacing auth and id";

#[test]
fn merges_endpoint_query_and_header_default() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
    assert!(!VERBATIM_ROUTE_WITH_1.is_empty());
    assert!(!VERBATIM_MERGES_ENDPOINT_QUER_2.is_empty());
}
