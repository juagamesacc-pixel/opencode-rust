// source: src/util/effect-http-client.ts — exports: withTransientReadRetry.
// PROVISIONAL pending effect http client: retry policy consts verbatim
// (retryOn errors-and-responses, times 2, exponential(200) + jittered).

/// source: retryOn "errors-and-responses" — verbatim.
pub const RETRY_ON: &str = "errors-and-responses";
/// source: times 2 — verbatim.
pub const RETRY_TIMES: u32 = 2;
/// source: Schedule.exponential(200) base ms — verbatim.
pub const RETRY_BASE_MS: u64 = 200;
/// source: Schedule.jittered — verbatim flag.
pub const RETRY_JITTERED: bool = true;

/// source: withTransientReadRetry policy descriptor — verbatim fields.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub retry_on: &'static str,
    pub times: u32,
    pub base_ms: u64,
    pub jittered: bool,
}

/// source: withTransientReadRetry() — verbatim policy.
pub fn with_transient_read_retry() -> RetryPolicy {
    RetryPolicy {
        retry_on: RETRY_ON,
        times: RETRY_TIMES,
        base_ms: RETRY_BASE_MS,
        jittered: RETRY_JITTERED,
    }
}
