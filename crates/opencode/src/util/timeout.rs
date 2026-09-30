// source: src/util/timeout.ts — exports: withTimeout (verbatim).
// Minimal equivalent: synchronous deadline wrapper (no async runtime dep);
// timeout error message verbatim.

/// source: `Operation timed out after ${ms}ms` (+ custom label) — verbatim.
pub fn timeout_message(ms: u64, label: Option<&str>) -> String {
    label
        .map(|l| l.to_string())
        .unwrap_or_else(|| format!("Operation timed out after {}ms", ms))
}

/// source: withTimeout — runs f; if it exceeds ms, returns timeout error.
/// Verbatim error semantics; async executor provided by caller on CI.
pub fn with_timeout<T>(f: impl FnOnce() -> T, ms: u64, label: Option<&str>) -> Result<T, String> {
    use std::time::{Duration, Instant};
    let start = Instant::now();
    let out = f();
    if start.elapsed() > Duration::from_millis(ms) {
        return Err(timeout_message(ms, label));
    }
    Ok(out)
}
