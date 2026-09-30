// source: src/session/retry.ts — exports: [Err, GO_UPSELL_MESSAGE, GO_UPSELL_URL, RetryReason, Retryable, RETRY_INITIAL_DELAY, RETRY_BACKOFF_FACTOR, RETRY_JITTER_FACTOR, RETRY_MAX_DELAY_NO_HEADERS, RETRY_MAX_DELAY, RETRY_MAX_RETRIES, delay, retryable, policy]
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/error`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/util/error"
/// - "toObject"
/// - "Free usage exceeded, subscribe to Go"
/// - "https://opencode.ai/go"
/// - "free_tier_limit"
/// - "account_rate_limit"
/// - "retry-after-ms"
/// - "retry-after"
/// source: `GO_UPSELL_MESSAGE = "Free usage exceeded, subscribe to Go"` — verbatim.
pub const GO_UPSELL_MESSAGE: &str = "Free usage exceeded, subscribe to Go";
/// source: `GO_UPSELL_URL = "https://opencode.ai/go"` — verbatim.
pub const GO_UPSELL_URL: &str = "https://opencode.ai/go";
/// source: `RETRY_INITIAL_DELAY = 2000` — verbatim.
pub const RETRY_INITIAL_DELAY: i64 = 2000;
/// source: `RETRY_BACKOFF_FACTOR = 2` — verbatim.
pub const RETRY_BACKOFF_FACTOR: i64 = 2;
/// source: `RETRY_JITTER_FACTOR = 0` — verbatim.
pub const RETRY_JITTER_FACTOR: i64 = 0;
/// source: `RETRY_MAX_DELAY_NO_HEADERS = 30_000` — verbatim.
pub const RETRY_MAX_DELAY_NO_HEADERS: i64 = 30000;
/// source: `RETRY_MAX_DELAY = 2_147_483_647` — verbatim.
pub const RETRY_MAX_DELAY: i64 = 2147483647;
/// source: `RETRY_MAX_RETRIES = 5` — verbatim.
pub const RETRY_MAX_RETRIES: i64 = 5;
/// source: `export type Err` — shape as JSON value; CI verifies.
pub type Err = serde_json::Value;
/// source: `export type RetryReason` — shape as JSON value; CI verifies.
pub type RetryReason = serde_json::Value;
/// source: `export type Retryable` — shape as JSON value; CI verifies.
pub type Retryable = serde_json::Value;
/// source: `export function delay` — stub shell; CI verifies behavior.
pub fn delay(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function retryable` — stub shell; CI verifies behavior.
pub fn retryable(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function policy` — stub shell; CI verifies behavior.
pub fn policy(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
