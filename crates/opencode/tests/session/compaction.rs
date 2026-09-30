// source: test/session/compaction.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { afterEach, describe, expect, mock, test } from "bun:test"; import { ConfigV1 } from "@opencode-ai/core/v1/confi

#[test]
fn session_compaction_is_overflow() {
    // source: "session.compaction.isOverflow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_when_token_count_exceeds_usable_context() {
    // source: "returns true when token count exceeds usable context"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_when_token_count_within_usable_context() {
    // source: "returns false when token count within usable context"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn includes_cache_read_in_token_count() {
    // source: "includes cache.read in token count"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn respects_input_limit_for_input_caps() {
    // source: "respects input limit for input caps"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_when_input_output_are_within_input_caps() {
    // source: "returns false when input/output are within input caps"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_when_output_within_limit_with_input_caps() {
    // source: "returns false when output within limit with input caps"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn bug_no_headroom_when_limit_input_is_set_compaction_should_tr() {
    // source: "BUG: no headroom when limit.input is set — compaction should trigger near boundary but does not"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn bug_without_limit_input_same_token_count_correctly_triggers_() {
    // source: "BUG: without limit.input, same token count correctly triggers compaction"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn bug_asymmetry_limit_input_model_allows_30_k_more_usage_befor() {
    // source: "BUG: asymmetry — limit.input model allows 30K more usage before compaction than equivalent model without it"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_when_model_context_limit_is_0() {
    // source: "returns false when model context limit is 0"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_when_compaction_auto_is_disabled() {
    // source: "returns false when compaction.auto is disabled"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_compaction_create() {
    // source: "session.compaction.create"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn creates_a_compaction_user_message_and_part() {
    // source: "creates a compaction user message and part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_compaction_prune() {
    // source: "session.compaction.prune"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn compacts_old_completed_tool_output() {
    // source: "compacts old completed tool output"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_protected_skill_tool_output() {
    // source: "skips protected skill tool output"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_compaction_process() {
    // source: "session.compaction.process"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn throws_when_parent_is_not_a_user_message() {
    // source: "throws when parent is not a user message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn publishes_compacted_event_on_continue() {
    // source: "publishes compacted event on continue"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn adds_synthetic_continue_prompt_when_auto_is_enabled() {
    // source: "adds synthetic continue prompt when auto is enabled"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_the_prior_user_turn_on_overflow_when_earlier_context() {
    // source: "replays the prior user turn on overflow when earlier context exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn falls_back_to_overflow_guidance_when_no_replayable_turn_exis() {
    // source: "falls back to overflow guidance when no replayable turn exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn util_token_estimate() {
    // source: "util.token.estimate"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn estimates_tokens_from_text_4_chars_per_token() {
    // source: "estimates tokens from text (4 chars per token)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn estimates_tokens_from_larger_text() {
    // source: "estimates tokens from larger text"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_0_for_empty_string() {
    // source: "returns 0 for empty string"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_ns_get_usage() {
    // source: "SessionNs.getUsage"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn normalizes_standard_usage_to_token_format() {
    // source: "normalizes standard usage to token format"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn extracts_cached_tokens_to_cache_read() {
    // source: "extracts cached tokens to cache.read"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_anthropic_cache_write_metadata() {
    // source: "handles anthropic cache write metadata"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn subtracts_cached_tokens_for_anthropic_provider() {
    // source: "subtracts cached tokens for anthropic provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn separates_reasoning_tokens_from_output_tokens() {
    // source: "separates reasoning tokens from output tokens"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_double_count_reasoning_tokens_in_cost() {
    // source: "does not double count reasoning tokens in cost"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_undefined_optional_values_gracefully() {
    // source: "handles undefined optional values gracefully"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_malformed_cost_fields() {
    // source: "ignores malformed cost fields"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn calculates_cost_correctly() {
    // source: "calculates cost correctly"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_authoritative_copilot_billed_cost_when_provided() {
    // source: "uses authoritative Copilot billed cost when provided"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_matching_context_cost_tier_before_over_200k_fallback() {
    // source: "uses matching context cost tier before over-200k fallback"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn falls_back_to_over_200k_pricing_when_no_cost_tier_matches() {
    // source: "falls back to over-200k pricing when no cost tier matches"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn extracts_cache_write_tokens_from_vertex_metadata_key() {
    // source: "extracts cache write tokens from vertex metadata key"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
