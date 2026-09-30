//! Rust port of `packages/app/src/utils/uuid.ts` (opencode v1.18.30).
//!
//! Source 12 lines: `uuid()` uses `crypto.randomUUID` when a secure context is
//! available, otherwise falls back to `Math.random().toString(16).slice(2)`.
//!
//! 1:1 notes:
//! - `globalThis.crypto` / `Math.random` are browser globals; this port models
//!   them as a thread-local `UuidRuntime` (see `UuidTesting`) so the 1:1 tests
//!   (`uuid.test.ts`) can drive every branch. The default runtime represents a
//!   non-browser host: no `crypto`, and a time-seeded LCG standing in for
//!   `Math.random`.
//! - The secure-context guard runs *before* the `try` block, exactly like the
//!   source (`!crypto?.randomUUID || !crypto?.isSecureContext` → fallback).
//! - Original file: `packages/app/src/utils/uuid.ts`

#![allow(dead_code)]

use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

/// Mirrors the shape of `globalThis.crypto`: `None` means `crypto` is absent.
#[derive(Debug, Clone, Copy)]
pub enum CryptoRandomUuid {
    /// `crypto.randomUUID` exists and returns normally.
    Returns(fn() -> String),
    /// `crypto.randomUUID` exists but throws.
    Throws,
}

#[derive(Debug, Clone, Copy)]
pub struct UuidRuntime {
    pub crypto_random_uuid: Option<CryptoRandomUuid>,
    pub is_secure_context: Option<bool>,
    pub math_random: fn() -> f64,
}

impl Default for UuidRuntime {
    fn default() -> Self {
        UuidRuntime {
            crypto_random_uuid: None,
            is_secure_context: None,
            math_random: default_random,
        }
    }
}

thread_local! {
    static UUID_RUNTIME: RefCell<UuidRuntime> =
        RefCell::new(UuidRuntime { crypto_random_uuid: None, is_secure_context: None, math_random: default_random });
}

/// Mirrors `uuid()`.
pub fn uuid() -> String {
    let runtime = UUID_RUNTIME.with(|r| *r.borrow());
    let generate = match runtime.crypto_random_uuid {
        Some(CryptoRandomUuid::Returns(f)) if runtime.is_secure_context == Some(true) => Some(f),
        _ => None,
    };
    match generate {
        Some(f) => f(),
        None => fallback(runtime.math_random),
    }
}

/// Mirrors `Math.random().toString(16).slice(2)`: the hex digits of the
/// fractional part (`0.5` → `"8"`).
fn fallback(random: fn() -> f64) -> String {
    random_hex_fraction(random())
}

fn random_hex_fraction(x: f64) -> String {
    let mut x = x;
    let mut out = String::new();
    for _ in 0..13 {
        x *= 16.0;
        let digit = (x.floor() as u32).min(15);
        out.push(char::from_digit(digit, 16).expect("digit < 16"));
        x -= digit as f64;
        if x == 0.0 {
            break;
        }
    }
    out
}

/// Time-seeded LCG standing in for `Math.random` (PROVISIONAL: browser global).
fn default_random() -> f64 {
    static SEED: AtomicU64 = AtomicU64::new(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let x = SEED
        .fetch_add(1, Ordering::Relaxed)
        .wrapping_mul(2_654_435_761)
        .wrapping_add(now);
    (x >> 8) as f64 / (1u64 << 56) as f64
}

/// Test hook mirroring reassigning `globalThis.crypto` / `Math.random` in
/// `uuid.test.ts`. Returns the previous runtime for restoration.
pub mod uuid_testing {
    use super::*;

    pub fn set_props(
        crypto: Option<CryptoRandomUuid>,
        is_secure_context: Option<bool>,
        math_random: Option<fn() -> f64>,
    ) -> UuidRuntime {
        UUID_RUNTIME.with(|r| {
            let mut r = r.borrow_mut();
            let prev = *r;
            r.crypto_random_uuid = crypto;
            r.is_secure_context = is_secure_context;
            if let Some(random) = math_random {
                r.math_random = random;
            }
            prev
        })
    }

    pub fn restore(runtime: UuidRuntime) {
        UUID_RUNTIME.with(|r| {
            *r.borrow_mut() = runtime;
        });
    }
}
