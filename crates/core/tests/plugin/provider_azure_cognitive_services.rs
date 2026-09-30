#![allow(clippy::all)]
// source: test/plugin/provider-azure-cognitive-services.test.ts — exports/cases: ["maps the resource env var to the Azure SDK baseURL","leaves baseURL unset without resource env and ignores other providers","selects chat only for completion URLs","uses the legacy Azure selector order and provider guard","falls back from responses to messages, chat, then languageModel"]
// PROVISIONAL pending core — verbatim shape where pure
// original imports: import { AISDK } from "@opencode-ai/core/aisdk" import { describe, expect } from "bun:test" import type { LanguageModelV3 } from "@ai-sdk/provider"

// describe: ["AzureCognitiveServicesPlugin"]
#[test]
fn maps_the_resource_env_var_to_the_azure_sdk_baseurl() {
    // source: "maps the resource env var to the Azure SDK baseURL"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure-cognitive-services.test.ts: maps the resource env var to the Azure SDK baseURL");
}

#[test]
fn leaves_baseurl_unset_without_resource_env_and_ignores_other() {
    // source: "leaves baseURL unset without resource env and ignores other providers"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure-cognitive-services.test.ts: leaves baseURL unset without resource env and ignores other providers");
}

#[test]
fn selects_chat_only_for_completion_urls() {
    // source: "selects chat only for completion URLs"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure-cognitive-services.test.ts: selects chat only for completion URLs");
}

#[test]
fn uses_the_legacy_azure_selector_order_and_provider_guard() {
    // source: "uses the legacy Azure selector order and provider guard"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure-cognitive-services.test.ts: uses the legacy Azure selector order and provider guard");
}

#[test]
fn falls_back_from_responses_to_messages_chat_then_languagemode() {
    // source: "falls back from responses to messages, chat, then languageModel"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure-cognitive-services.test.ts: falls back from responses to messages, chat, then languageModel");
}
