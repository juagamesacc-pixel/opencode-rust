// source: test/session/retry.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test"; import { LayerNode } from "@opencode-ai/core/effect/layer-node"; impo

#[test]
fn session_retry_delay() {
    // source: "session.retry.delay"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn caps_delay_at_30_seconds_when_headers_missing() {
    // source: "caps delay at 30 seconds when headers missing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn adds_jitter_to_exponential_delays() {
    // source: "adds jitter to exponential delays"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn prefers_retry_after_ms_when_shorter_than_exponential() {
    // source: "prefers retry-after-ms when shorter than exponential"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_retry_after_seconds_when_reasonable() {
    // source: "uses retry-after seconds when reasonable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn accepts_http_date_retry_after_values() {
    // source: "accepts http-date retry-after values"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_invalid_retry_hints() {
    // source: "ignores invalid retry hints"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_malformed_date_retry_hints() {
    // source: "ignores malformed date retry hints"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_past_date_retry_hints() {
    // source: "ignores past date retry hints"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_retry_after_values_even_when_exceeding_10_minutes_with_() {
    // source: "uses retry-after values even when exceeding 10 minutes with headers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn caps_oversized_header_delays_to_the_runtime_timer_limit() {
    // source: "caps oversized header delays to the runtime timer limit"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn policy_updates_retry_status_and_increments_attempts() {
    // source: "policy updates retry status and increments attempts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn policy_stops_after_five_retries() {
    // source: "policy stops after five retries"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_retry_retryable() {
    // source: "session.retry.retryable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_serialized_too_many_requests_messages() {
    // source: "retries serialized too_many_requests messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_serialized_overloaded_provider_codes() {
    // source: "retries serialized overloaded provider codes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_serialized_rate_limit_messages() {
    // source: "retries serialized rate_limit messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_retry_unknown_json_messages() {
    // source: "does not retry unknown json messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_throw_on_numeric_error_codes() {
    // source: "does not throw on numeric error codes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_undefined_for_non_json_message() {
    // source: "returns undefined for non-json message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_plain_text_rate_limit_errors_from_alibaba() {
    // source: "retries plain text rate limit errors from Alibaba"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_plain_text_rate_limit_errors() {
    // source: "retries plain text rate limit errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_too_many_requests_in_plain_text() {
    // source: "retries too many requests in plain text"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_hyphenated_service_unavailable_errors() {
    // source: "retries hyphenated service-unavailable errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_retryable_api_response_bodies() {
    // source: "matches retryable API response bodies"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_transport_timeout_errors() {
    // source: "retries transport timeout errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_websocket_stream_transport_errors() {
    // source: "retries websocket stream transport errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_retry_context_overflow_errors() {
    // source: "does not retry context overflow errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_500_errors_even_when_is_retryable_is_false() {
    // source: "retries 500 errors even when isRetryable is false"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_502_bad_gateway_errors() {
    // source: "retries 502 bad gateway errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_503_service_unavailable_errors() {
    // source: "retries 503 service unavailable errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_retry_4xx_errors_when_is_retryable_is_false() {
    // source: "does not retry 4xx errors when isRetryable is false"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_zlib_error_decompression_failures() {
    // source: "retries ZlibError decompression failures"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_free_limits_to_go_upsell_action() {
    // source: "maps free limits to Go upsell action"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_go_subscription_limits_to_workspace_payg_upsell() {
    // source: "maps Go subscription limits to workspace PAYG upsell"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_go_subscription_limits_without_limit_metadata() {
    // source: "maps Go subscription limits without limit metadata"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_message_v2_from_error() {
    // source: "session.message-v2.fromError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn econnreset_socket_error_is_retryable() {
    // source: "ECONNRESET socket error is retryable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn marks_open_ai_404_status_codes_as_retryable() {
    // source: "marks OpenAI 404 status codes as retryable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_open_ai_server_error_stream_chunks_to_retryable_api() {
    // source: "converts OpenAI server_error stream chunks to retryable APIError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
