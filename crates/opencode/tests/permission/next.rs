// source: test/permission/next.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { PermissionV1 } from "@opencode-ai/core/v1/permission"; import { test, expect } from "bun:test"; import os from

#[test]
fn from_config_string_value_becomes_wildcard_rule() {
    // source: "fromConfig - string value becomes wildcard rule"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_object_value_converts_to_rules_array() {
    // source: "fromConfig - object value converts to rules array"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_mixed_string_and_object_values() {
    // source: "fromConfig - mixed string and object values"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_empty_object() {
    // source: "fromConfig - empty object"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_expands_tilde_to_home_directory() {
    // source: "fromConfig - expands tilde to home directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_expands_home_to_home_directory() {
    // source: "fromConfig - expands $HOME to home directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_expands_home_without_trailing_slash() {
    // source: "fromConfig - expands $HOME without trailing slash"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_does_not_expand_tilde_in_middle_of_path() {
    // source: "fromConfig - does not expand tilde in middle of path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_preserves_top_level_config_key_order() {
    // source: "fromConfig - preserves top-level config key order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_wildcard_acts_as_fallback_when_it_appears_before() {
    // source: "fromConfig - wildcard acts as fallback when it appears before specifics"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_top_level_ordering_is_not_sorted_by_wildcard_spe() {
    // source: "fromConfig - top-level ordering is not sorted by wildcard specificity"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_sub_pattern_insertion_order_inside_a_tool_key_is() {
    // source: "fromConfig - sub-pattern insertion order inside a tool key is preserved"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_documented_fallback_first_example() {
    // source: "fromConfig - documented fallback-first example"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn from_config_expands_exact_tilde_to_home_directory() {
    // source: "fromConfig - expands exact tilde to home directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_matches_expanded_tilde_pattern() {
    // source: "evaluate - matches expanded tilde pattern"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_matches_expanded_home_pattern() {
    // source: "evaluate - matches expanded $HOME pattern"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_simple_concatenation() {
    // source: "merge - simple concatenation"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_adds_new_permission() {
    // source: "merge - adds new permission"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_concatenates_rules_for_same_permission() {
    // source: "merge - concatenates rules for same permission"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_multiple_rulesets() {
    // source: "merge - multiple rulesets"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_empty_ruleset_does_nothing() {
    // source: "merge - empty ruleset does nothing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_preserves_rule_order() {
    // source: "merge - preserves rule order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_config_permission_overrides_default_ask() {
    // source: "merge - config permission overrides default ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_config_ask_overrides_default_allow() {
    // source: "merge - config ask overrides default allow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_exact_pattern_match() {
    // source: "evaluate - exact pattern match"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_wildcard_pattern_match() {
    // source: "evaluate - wildcard pattern match"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_last_matching_rule_wins() {
    // source: "evaluate - last matching rule wins"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_last_matching_rule_wins_wildcard_after_specific() {
    // source: "evaluate - last matching rule wins (wildcard after specific)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_glob_pattern_match() {
    // source: "evaluate - glob pattern match"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_last_matching_glob_wins() {
    // source: "evaluate - last matching glob wins"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_order_matters_for_specificity() {
    // source: "evaluate - order matters for specificity"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_unknown_permission_returns_ask() {
    // source: "evaluate - unknown permission returns ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_empty_ruleset_returns_ask() {
    // source: "evaluate - empty ruleset returns ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_no_matching_pattern_returns_ask() {
    // source: "evaluate - no matching pattern returns ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_empty_rules_array_returns_ask() {
    // source: "evaluate - empty rules array returns ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_multiple_matching_patterns_last_wins() {
    // source: "evaluate - multiple matching patterns, last wins"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_non_matching_patterns_are_skipped() {
    // source: "evaluate - non-matching patterns are skipped"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_exact_match_at_end_wins_over_earlier_wildcard() {
    // source: "evaluate - exact match at end wins over earlier wildcard"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_wildcard_at_end_overrides_earlier_exact_match() {
    // source: "evaluate - wildcard at end overrides earlier exact match"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_wildcard_permission_matches_any_permission() {
    // source: "evaluate - wildcard permission matches any permission"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_wildcard_permission_with_specific_pattern() {
    // source: "evaluate - wildcard permission with specific pattern"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_glob_permission_pattern() {
    // source: "evaluate - glob permission pattern"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_specific_permission_and_wildcard_permission_combine() {
    // source: "evaluate - specific permission and wildcard permission combined"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_wildcard_permission_does_not_match_when_specific_ex() {
    // source: "evaluate - wildcard permission does not match when specific exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_multiple_matching_permission_patterns_combine_rules() {
    // source: "evaluate - multiple matching permission patterns combine rules"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_wildcard_permission_fallback_for_unknown_tool() {
    // source: "evaluate - wildcard permission fallback for unknown tool"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_later_wildcard_permission_can_override_earlier_spec() {
    // source: "evaluate - later wildcard permission can override earlier specific permission"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_merges_multiple_rulesets() {
    // source: "evaluate - merges multiple rulesets"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_returns_empty_set_when_all_tools_allowed() {
    // source: "disabled - returns empty set when all tools allowed"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_disables_tool_when_denied() {
    // source: "disabled - disables tool when denied"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_disables_edit_write_apply_patch_when_edit_denied() {
    // source: "disabled - disables edit/write/apply_patch when edit denied"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_does_not_disable_when_partially_denied() {
    // source: "disabled - does not disable when partially denied"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_does_not_disable_when_action_is_ask() {
    // source: "disabled - does not disable when action is ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_does_not_disable_when_specific_allow_after_wildcard() {
    // source: "disabled - does not disable when specific allow after wildcard deny"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_does_not_disable_when_wildcard_allow_after_deny() {
    // source: "disabled - does not disable when wildcard allow after deny"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_disables_multiple_tools() {
    // source: "disabled - disables multiple tools"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_wildcard_permission_denies_all_tools() {
    // source: "disabled - wildcard permission denies all tools"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_specific_allow_overrides_wildcard_deny() {
    // source: "disabled - specific allow overrides wildcard deny"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_resolves_immediately_when_action_is_allow() {
    // source: "ask - resolves immediately when action is allow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_throws_denied_error_when_action_is_deny() {
    // source: "ask - throws DeniedError when action is deny"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_stays_pending_when_action_is_ask() {
    // source: "ask - stays pending when action is ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_adds_request_to_pending_list() {
    // source: "ask - adds request to pending list"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_publishes_asked_event() {
    // source: "ask - publishes asked event"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_once_resolves_the_pending_ask() {
    // source: "reply - once resolves the pending ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_reject_throws_rejected_error() {
    // source: "reply - reject throws RejectedError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_reject_with_message_throws_corrected_error() {
    // source: "reply - reject with message throws CorrectedError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_always_persists_approval_and_resolves() {
    // source: "reply - always persists approval and resolves"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_reject_cancels_all_pending_for_same_session() {
    // source: "reply - reject cancels all pending for same session"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_always_resolves_matching_pending_requests_in_same_sess() {
    // source: "reply - always resolves matching pending requests in same session"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_always_keeps_other_session_pending() {
    // source: "reply - always keeps other session pending"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_publishes_replied_event() {
    // source: "reply - publishes replied event"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn permission_requests_stay_isolated_by_directory() {
    // source: "permission requests stay isolated by directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn pending_permission_rejects_on_instance_dispose() {
    // source: "pending permission rejects on instance dispose"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn pending_permission_rejects_on_instance_reload() {
    // source: "pending permission rejects on instance reload"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reply_fails_for_unknown_request_id() {
    // source: "reply - fails for unknown requestID"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_checks_all_patterns_and_stops_on_first_deny() {
    // source: "ask - checks all patterns and stops on first deny"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_allows_all_patterns_when_all_match_allow_rules() {
    // source: "ask - allows all patterns when all match allow rules"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_should_deny_even_when_an_earlier_pattern_is_ask() {
    // source: "ask - should deny even when an earlier pattern is ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ask_abort_should_clear_pending_request() {
    // source: "ask - abort should clear pending request"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
