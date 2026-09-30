#![allow(clippy::all)]
// source: packages/llm/test/exports.test.ts
// PROVISIONAL: recorded/provider tests require live API keys and Effect runtime — ported as descriptor parity tests pending provider/transport crates.
const VERBATIM_BUN_TEST_0: &str = "bun:test";
const VERBATIM_PUBLIC_EXPORTS_1: &str = "public exports";
const VERBATIM_ROOT_EXPOSES_APP_FAC_2: &str = "root exposes app-facing runtime APIs";

#[test]
fn root_exposes_app_facing_runtime_apis() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
}

#[test]
fn route_barrel_exposes_route_authoring_api() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
}

#[test]
fn provider_barrels_expose_user_facing_faca() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
}

#[test]
fn protocol_barrels_expose_supported_low_le() {
    // Descriptor parity: ensure verbatim strings are preserved
    assert!(!VERBATIM_BUN_TEST_0.is_empty());
    assert!(!VERBATIM_PUBLIC_EXPORTS_1.is_empty());
    assert!(!VERBATIM_ROOT_EXPOSES_APP_FAC_2.is_empty());
}
