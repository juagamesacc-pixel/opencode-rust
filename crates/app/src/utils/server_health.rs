//! Rust port of `packages/app/src/utils/server-health.ts` (opencode v1.18.30).
//!
//! Source 172 lines: `ServerHealth`, `checkServerHealth` (timeout/retry/
//! V1-fallback), `useCheckServerHealth` polling. Network clients are
//! PROVISIONAL; timeout/retry defaults and endpoint order are verbatim.
//! Original file: `packages/app/src/utils/server-health.ts`

#![allow(dead_code)]

/// Mirrors `ServerHealth`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ServerHealth {
    pub healthy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Mirrors `CheckServerHealthOptions` defaults (verbatim).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckServerHealthOptions {
    pub timeout_ms: u64,
    pub retry_count: u32,
    pub retry_delay_ms: u64,
}

impl Default for CheckServerHealthOptions {
    fn default() -> Self {
        Self {
            timeout_ms: 30_000,
            retry_count: 2,
            retry_delay_ms: 100,
        }
    }
}

/// Mirrors the health-cache TTL + poll interval (verbatim).
pub const HEALTH_CACHE_MS: u64 = 750;
pub const HEALTH_POLL_MS: u64 = 10_000;

/// Mirrors the `retryable(error, signal?)` predicate over error names.
pub fn health_error_retryable(
    name: Option<&str>,
    message: Option<&str>,
    is_type_error: bool,
) -> bool {
    if let Some(name) = name {
        if name == "AbortError" || name == "TimeoutError" {
            return false;
        }
    }
    if is_type_error {
        return true;
    }
    match message {
        None => false,
        Some(message) => {
            let lower = message.to_lowercase();
            [
                "network",
                "fetch",
                "econnreset",
                "econnrefused",
                "enotfound",
                "timedout",
            ]
            .iter()
            .any(|needle| lower.contains(needle))
        }
    }
}

/// Mirrors the retry-delay schedule (`retryDelayMs * (count + 1)`).
pub fn health_retry_delay_ms(options: &CheckServerHealthOptions, count: u64) -> u64 {
    options.retry_delay_ms * (count + 1)
}

// PROVISIONAL: pending client/network runtime — mirrors `packages/app/src/utils/server-health.ts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerHealthProbe {
    pub options: CheckServerHealthOptions,
    pub attempts: u32,
}

impl ServerHealthProbe {
    pub fn new(options: CheckServerHealthOptions) -> Self {
        Self {
            options,
            attempts: 0,
        }
    }

    pub fn update_attempt(&mut self) {
        self.attempts += 1;
    }

    pub fn transition_should_retry(&self, retryable: bool) -> bool {
        retryable && self.attempts <= self.options.retry_count
    }
}
