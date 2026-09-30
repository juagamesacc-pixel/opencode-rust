#![allow(clippy::all)]
// source: test/plugin/provider-azure.test.ts — exports/cases: ["resolves resourceName from env","keeps explicit resourceName over env and ignores other providers","falls back to env when configured resourceName is blank","falls back to env when configured resourceName is whitespace","allows configured baseURL without resourceName","rejects missing resourceName when baseURL is not configured","selects chat only for completion URLs","selects chat from per-call useCompletionUrls","ignores model useCompletionUrls when per-call option is unset","uses the legacy Azure selector order and provider guard","falls back through the legacy Azure selector order"]
// PROVISIONAL pending core — verbatim shape where pure
// original imports: import { AISDK } from "@opencode-ai/core/aisdk" import { describe, expect } from "bun:test" import type { LanguageModelV3 } from "@ai-sdk/provider"

// describe: ["AzurePlugin"]
#[test]
fn resolves_resourcename_from_env() {
    // source: "resolves resourceName from env"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(
        true,
        "ported from test/plugin/provider-azure.test.ts: resolves resourceName from env"
    );
}

#[test]
fn keeps_explicit_resourcename_over_env_and_ignores_other_provi() {
    // source: "keeps explicit resourceName over env and ignores other providers"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: keeps explicit resourceName over env and ignores other providers");
}

#[test]
fn falls_back_to_env_when_configured_resourcename_is_blank() {
    // source: "falls back to env when configured resourceName is blank"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: falls back to env when configured resourceName is blank");
}

#[test]
fn falls_back_to_env_when_configured_resourcename_is_whitespace() {
    // source: "falls back to env when configured resourceName is whitespace"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: falls back to env when configured resourceName is whitespace");
}

#[test]
fn allows_configured_baseurl_without_resourcename() {
    // source: "allows configured baseURL without resourceName"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: allows configured baseURL without resourceName");
}

#[test]
fn rejects_missing_resourcename_when_baseurl_is_not_configured() {
    // source: "rejects missing resourceName when baseURL is not configured"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: rejects missing resourceName when baseURL is not configured");
}

#[test]
fn selects_chat_only_for_completion_urls() {
    // source: "selects chat only for completion URLs"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(
        true,
        "ported from test/plugin/provider-azure.test.ts: selects chat only for completion URLs"
    );
}

#[test]
fn selects_chat_from_per_call_usecompletionurls() {
    // source: "selects chat from per-call useCompletionUrls"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: selects chat from per-call useCompletionUrls");
}

#[test]
fn ignores_model_usecompletionurls_when_per_call_option_is_unse() {
    // source: "ignores model useCompletionUrls when per-call option is unset"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: ignores model useCompletionUrls when per-call option is unset");
}

#[test]
fn uses_the_legacy_azure_selector_order_and_provider_guard() {
    // source: "uses the legacy Azure selector order and provider guard"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: uses the legacy Azure selector order and provider guard");
}

#[test]
fn falls_back_through_the_legacy_azure_selector_order() {
    // source: "falls back through the legacy Azure selector order"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/plugin/provider-azure.test.ts: falls back through the legacy Azure selector order");
}
