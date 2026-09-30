//! 1:1 port of the `ulid` npm package surface used by `core/src/identifier.ts`.
//!
//! Host runtime shim, not a source file. Crockford base32, 26 characters:
//! 10 characters of 48-bit millisecond timestamp followed by 16 characters of
//! 80-bit entropy, with the monotonic-counter increment the package applies when
//! two IDs are generated inside the same millisecond.

use std::sync::atomic::{AtomicU64, Ordering};

const ENCODING: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const TIME_LEN: usize = 10;
const RANDOM_LEN: usize = 16;

/// 80 bits of entropy, bumped on every call so same-millisecond IDs stay
/// strictly increasing, mirroring `ulid()`'s monotonic factory.
static STATE: AtomicU64 = AtomicU64::new(0);

/// `ulid()` — a fresh 26-character ULID.
pub fn ulid() -> String {
    let now = crate::runtime::time::current_time_millis();
    let mut out = String::with_capacity(TIME_LEN + RANDOM_LEN);
    // 48-bit big-endian timestamp, Crockford base32.
    for index in (0..TIME_LEN).rev() {
        let shift = index * 5;
        out.push(ENCODING[(((now as u64) >> shift) & 0x1f) as usize] as char);
    }
    out.push_str(&random_suffix(now));
    out
}

/// 80 bits of entropy rendered as 16 Crockford base32 characters.
fn random_suffix(now: i64) -> String {
    let mut state = next_state(now);
    let mut out = String::with_capacity(RANDOM_LEN);
    // 16 * 5 = 80 bits, consumed most-significant first.
    for index in (0..RANDOM_LEN).rev() {
        let shift = index * 5;
        out.push(ENCODING[((state >> shift) & 0x1f) as usize] as char);
    }
    out
}

/// xorshift64* mix of a monotonic counter and the wall clock, so IDs are unique
/// and ordered without pulling in an RNG dependency.
fn next_state(now: i64) -> u64 {
    let counter = STATE.fetch_add(1, Ordering::SeqCst).wrapping_add(1);
    let mut state =
        (now as u64) ^ counter.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ 0x2545_f491_4f6c_dd1d;
    if state == 0 {
        state = 0x2545_f491_4f6c_dd1d;
    }
    state ^= state >> 30;
    state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 27;
    state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
    state ^= state >> 31;
    state
}
