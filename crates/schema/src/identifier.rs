//! Port of `packages/schema/src/identifier.ts`.
//!
//! Sortable ID-suffix generator. The generation algorithm is ported exactly:
//! same string format, same counter behavior, same byte layout, same
//! modulo-62 alphabet mapping (including its modulo bias — not "fixed").
//!
//! Source algorithm (`create(descending, timestamp = Date.now())`):
//! - When `timestamp` differs from the last call, the per-timestamp `counter`
//!   resets to `0`; then `counter` increments.
//! - `current = BigInt(timestamp) * 0x1000n + BigInt(counter)`; when
//!   `descending`, `value = ~current`, else `value = current`.
//! - The first 12 chars are 6 big-endian bytes of `value` (`(value >>
//!   (40 - 8 * index)) & 0xff` for `index` in `0..6`), each as 2 lowercase hex
//!   chars. Only the low 48 bits are kept — the truncation is verbatim, not a
//!   bug to widen.
//! - The remaining `26 - 12` chars come from `crypto.getRandomValues` bytes
//!   mapped as `chars[byte % 62]` over
//!   `0-9A-Za-z`.
//!
//! Rust notes: `BigInt` arithmetic is `i128` here (values fit with room to
//! spare; `>>` on negatives is arithmetic, matching `BigInt` semantics for
//! the low byte). `crypto.getRandomValues` maps to the OS CSPRNG read via
//! std only (`/dev/urandom`; no new deps allowed) with a time-seeded fallback
//! when unavailable. Module state (`lastTimestamp`/`counter`) is a
//! mutex-guarded static, preserving single-shared-counter semantics.

use std::io::Read;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Total suffix length (`length = 26` in source).
const LENGTH: usize = 26;

/// Hex time prefix length in chars (6 bytes).
const TIME_CHARS: usize = 12;

/// Alphabet (`chars` in source): `0-9A-Za-z`, 62 entries.
const CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

struct IdentifierState {
    last_timestamp: i64,
    counter: u64,
}

static STATE: OnceLock<Mutex<IdentifierState>> = OnceLock::new();

fn state() -> &'static Mutex<IdentifierState> {
    STATE.get_or_init(|| {
        Mutex::new(IdentifierState {
            last_timestamp: 0,
            counter: 0,
        })
    })
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

/// Port of `crypto.getRandomValues(new Uint8Array(...))` using std only.
fn random_bytes(count: usize) -> Vec<u8> {
    if let Ok(mut file) = std::fs::File::open("/dev/urandom") {
        let mut bytes = vec![0u8; count];
        if file.read_exact(&mut bytes).is_ok() {
            return bytes;
        }
    }
    // Fallback when the OS source is unavailable: time-seeded xorshift.
    // Shape (count bytes) is preserved; only the entropy source differs.
    let mut seed = now_millis() as u64 ^ 0x9e3779b97f4a7c15u64;
    if seed == 0 {
        seed = 0x243f6a8885a308d3u64;
    }
    let mut bytes = Vec::with_capacity(count);
    for _ in 0..count {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        bytes.push((seed >> 32) as u8);
    }
    bytes
}

/// Port of `ascending()` (`identifier.ts`): `create(false)`.
pub fn ascending() -> String {
    create(false, None)
}

/// Port of `descending()` (`identifier.ts`): `create(true)`.
pub fn descending() -> String {
    create(true, None)
}

/// Port of `create(descending, timestamp = Date.now())` (`identifier.ts`).
///
/// `timestamp` defaults to the current epoch millis when `None` (port of the
/// `Date.now()` default parameter; Rust has no default parameters).
pub fn create(descending: bool, timestamp: Option<i64>) -> String {
    let timestamp = timestamp.unwrap_or_else(now_millis);
    let counter = {
        let mut guard = state().lock().expect("identifier: state lock poisoned");
        if timestamp != guard.last_timestamp {
            guard.last_timestamp = timestamp;
            guard.counter = 0;
        }
        guard.counter += 1;
        guard.counter
    };

    let current: i128 = (timestamp as i128) * 0x1000i128 + (counter as i128);
    let value: i128 = if descending { !current } else { current };
    let mut out = String::with_capacity(LENGTH);
    for index in 0..6 {
        let byte = ((value >> (40 - 8 * index)) & 0xff) as u8;
        out.push_str(&format!("{byte:02x}"));
    }
    debug_assert_eq!(out.len(), TIME_CHARS);
    let alphabet: Vec<char> = CHARS.chars().collect();
    for byte in random_bytes(LENGTH - TIME_CHARS) {
        out.push(alphabet[(byte % 62) as usize]);
    }
    out
}
