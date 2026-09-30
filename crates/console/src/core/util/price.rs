// source: core/src/util/price.ts
//! 1:1 port of the micro-cent conversions used by the `cost` bigint columns.
//! Source pin: v1.18.30 @3104c14.

use crate::runtime::num::round_int;

/// `centsToMicroCents(amount)` — `Math.round(amount * 1000000)`.
pub fn cents_to_micro_cents(amount: f64) -> i64 {
    round_int(amount * 1_000_000.0)
}

/// `microCentsToCents(amount)` — `Math.round(amount / 1000000)`.
pub fn micro_cents_to_cents(amount: f64) -> i64 {
    round_int(amount / 1_000_000.0)
}
