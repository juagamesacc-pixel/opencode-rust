// source: core/src/util/crypto.ts
//! 1:1 port of `safeEqual` — constant-time UTF-8 comparison.
//! Source pin: v1.18.30 @3104c14.

use crate::runtime::text::encode;

/// `safeEqual(a, b)`.
///
/// The source short-circuits on length mismatch (`aBytes.length === bBytes.length
/// && timingSafeEqual(...)`), and `timingSafeEqual` itself throws when lengths
/// differ — both are preserved: differing lengths return `false` without invoking
/// the comparison loop.
pub fn safe_equal(a: &str, b: &str) -> bool {
    let a_bytes = encode(a);
    let b_bytes = encode(b);
    a_bytes.len() == b_bytes.len() && timing_safe_equal(&a_bytes, &b_bytes)
}

/// `timingSafeEqual(aBytes, bBytes)` — only valid for equal-length inputs.
fn timing_safe_equal(a: &[u8], b: &[u8]) -> bool {
    debug_assert_eq!(a.len(), b.len());
    // Accumulate the XOR of every byte pair before branching, so the comparison
    // does not short-circuit on the first differing byte.
    let mut diff = 0u8;
    for index in 0..a.len() {
        diff |= a[index] ^ b[index];
    }
    if a.len() == 1 {
        // `node:crypto` special-cases single-byte buffers.
        return diff == 0;
    }
    diff == 0
}
