// source: test/plugin/xai.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test"; import { accessTokenIsExpiring, pollDeviceCodeToken, requestDeviceCod

#[test]
fn plugin_xai() {
    // source: "plugin.xai"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn access_token_is_expiring() {
    // source: "accessTokenIsExpiring"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_for_an_already_expired_jwt() {
    // source: "returns true for an already-expired JWT"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_a_fresh_jwt_outside_the_skew_window() {
    // source: "returns false for a fresh JWT outside the skew window"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn honors_the_skew_window() {
    // source: "honors the skew window"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn clamps_negative_skew_to_zero_rather_than_refusing_to_refresh() {
    // source: "clamps negative skew to zero rather than refusing to refresh"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_opaque_and_malformed_tokens() {
    // source: "returns false for opaque and malformed tokens"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loader() {
    // source: "loader"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_no_options_unless_stored_auth_is_oauth_and_exposes_m() {
    // source: "returns no options unless stored auth is OAuth and exposes methods in order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replaces_the_dummy_bearer_sets_user_agent_and_preserves_call() {
    // source: "replaces the dummy bearer, sets User-Agent, and preserves caller headers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_mutate_caller_headers_and_supports_headers_init_sha() {
    // source: "does not mutate caller headers and supports HeadersInit shapes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_headers_from_request_input_and_lets_init_headers_o() {
    // source: "preserves headers from Request input and lets init headers override them"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn falls_through_to_plain_fetch_when_stored_auth_flips_from_oau() {
    // source: "falls through to plain fetch when stored auth flips from oauth to api"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn deduplicates_concurrent_refreshes_within_a_loader_instance() {
    // source: "deduplicates concurrent refreshes within a loader instance"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_share_refresh_single_flight_across_loader_instances() {
    // source: "does not share refresh single-flight across loader instances"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn starts_a_new_refresh_after_success_and_clears_the_refresh_pr() {
    // source: "starts a new refresh after success and clears the refresh promise after failure"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_refresh_response_variants_and_persistence_failure() {
    // source: "handles refresh response variants and persistence failure"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn refreshes_based_on_stored_expiry_or_jwt_expiry_and_skips_ref() {
    // source: "refreshes based on stored expiry or JWT expiry and skips refresh when both are fresh"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn network_failure_during_refresh_surfaces_the_underlying_fetch() {
    // source: "network failure during refresh surfaces the underlying fetch error"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn device_code_flow() {
    // source: "device code flow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn authorize_advertises_verification_url_user_code_and_returns_() {
    // source: "authorize advertises verification URL + user code and returns success on callback"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn authorize_falls_back_to_verification_uri_when_verification_u() {
    // source: "authorize falls back to verification_uri when verification_uri_complete is absent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn request_device_code_posts_form_body_validates_fields_and_sur() {
    // source: "requestDeviceCode posts form body, validates fields, and surfaces endpoint errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn poll_device_code_token_resolves_on_success_and_posts_the_dev() {
    // source: "pollDeviceCodeToken resolves on success and posts the device-code grant"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn poll_device_code_token_honors_authorization_pending_and_slow() {
    // source: "pollDeviceCodeToken honors authorization_pending and slow_down"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn poll_device_code_token_handles_terminal_errors_and_timeout() {
    // source: "pollDeviceCodeToken handles terminal errors and timeout"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn poll_device_code_token_normalizes_bad_interval_and_expires_i() {
    // source: "pollDeviceCodeToken normalizes bad interval and expires_in values"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn device_code_authorize_callback_returns_failed_when_polling_e() {
    // source: "device-code authorize callback returns failed when polling errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
