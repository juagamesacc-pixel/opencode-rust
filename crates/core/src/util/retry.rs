// source: src/util/retry.ts — exports: RetryOptions, retry
//
// PROVISIONAL note: source `retry` is async (`Promise<T>`, `setTimeout`);
// this port is synchronous (`std::thread::sleep`) since the crate builds
// std-only. `error instanceof Error ? error.message : String(error)` maps to
// `E: Display` (our error types Display their JS message verbatim).

/// source: internal `TRANSIENT_MESSAGES` list — verbatim, same order.
pub const TRANSIENT_MESSAGES: [&str; 8] = [
    "load failed",
    "network connection was lost",
    "network request failed",
    "failed to fetch",
    "econnreset",
    "econnrefused",
    "etimedout",
    "socket hang up",
];

/// source: `isTransientError(error)` — message lowercased and substring-tested
/// against TRANSIENT_MESSAGES; falsy error -> false.
pub fn is_transient_error(message: &str) -> bool {
    if message.is_empty() {
        return false;
    }
    let lower = message.to_lowercase();
    TRANSIENT_MESSAGES.iter().any(|m| lower.contains(m))
}

/// source: `RetryOptions` with `attempts = 3, delay = 500, factor = 2,
/// maxDelay = 10000, retryIf = isTransientError` defaults.
pub struct RetryOptions<'a, E> {
    pub attempts: usize,
    pub delay_ms: u64,
    pub factor: u64,
    pub max_delay_ms: u64,
    pub retry_if: Option<&'a dyn Fn(&E) -> bool>,
}

impl<E> Default for RetryOptions<'_, E> {
    fn default() -> Self {
        RetryOptions {
            attempts: 3,
            delay_ms: 500,
            factor: 2,
            max_delay_ms: 10000,
            retry_if: None,
        }
    }
}

/// Default equivalent of `retryIf = isTransientError` for `E: Display`
/// (message via `String(error)`).
pub fn default_retry_if<E: std::fmt::Display>(error: &E) -> bool {
    is_transient_error(&error.to_string())
}

/// source: `retry(fn, options)` — attempts, then exponential backoff
/// `min(delay * factor^attempt, maxDelay)`; rethrows when the last attempt
/// fails or the error is not transient. The `throw lastError` tail is
/// unreachable (loop always returns or throws) — mirrored structurally.
pub fn retry<T, E, F>(mut f: F, options: RetryOptions<'_, E>) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    // NOTE: `attempts = 0` would `throw undefined` in JS; clamping to `1` is
    // the only divergence (invalid input only — cf. cli port's pid>0 guard).
    let attempts = options.attempts.max(1);
    let mut last_error: Option<E> = None;
    for attempt in 0..attempts {
        match f() {
            Ok(value) => return Ok(value),
            Err(error) => {
                let transient = match options.retry_if {
                    Some(retry_if) => retry_if(&error),
                    None => default_retry_if(&error),
                };
                last_error = Some(error);
                if attempt == attempts - 1 || !transient {
                    return Err(last_error.expect("last_error set on error path"));
                }
                let wait = options
                    .delay_ms
                    .saturating_mul(options.factor.pow(attempt as u32))
                    .min(options.max_delay_ms);
                std::thread::sleep(std::time::Duration::from_millis(wait));
            }
        }
    }
    Err(last_error.expect("loop always sets last_error before exit"))
}
