// source: src/util/hash.ts — exports: Hash (fast, sha256)
//
// PROVISIONAL crypto support: `createHash` from node:crypto is re-implemented
// in pure Rust (SHA-1 / SHA-256). Also hosts pub(crate) entropy primitives
// used by the node:crypto/`Math.random` equivalents of flock/effect-flock/
// slug/ticket (entropy from `/dev/urandom` with a time-seeded fallback).
//
// NOTE on naming: TS uses `import { Hash } from "./hash"` then `Hash.fast` /
// `Hash.sha256`; the Rust module `util::hash` exposes `fast` / `sha256`
// directly (module == namespace).

use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

/// source: `Hash.fast(input)` — sha1 hex of the input (utf8 string or Buffer
/// bytes).
pub fn fast(input: &[u8]) -> String {
    sha1_hex(input)
}

/// source: `Hash.sha256(input)` — sha256 hex digest.
pub fn sha256(input: &[u8]) -> String {
    let digest = crate::util::encode::hash_sha256_bytes(input);
    hex(&digest)
}

/// source: `Hash.fast` over a string (utf8 bytes).
pub fn fast_str(input: &str) -> String {
    sha1_hex(input.as_bytes())
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

// ---------------------------------------------------------------------------
// Pure-Rust SHA-1 (node:crypto createHash("sha1") equivalent)
// ---------------------------------------------------------------------------

fn sha1_hex(data: &[u8]) -> String {
    const H0: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    let mut h = H0;
    for block in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for (i, &wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5a827999u32),
                20..=39 => (b ^ c ^ d, 0x6ed9eba1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1bbcdc),
                _ => (b ^ c ^ d, 0xca62c1d6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    let mut out = Vec::with_capacity(20);
    for v in h {
        out.extend_from_slice(&v.to_be_bytes());
    }
    hex(&out)
}

// ---------------------------------------------------------------------------
// PROVISIONAL entropy: `Math.random()` / `crypto.randomUUID()` equivalents
// used by flock.ts / effect-flock.ts / slug.ts / pty/ticket.ts.
// ---------------------------------------------------------------------------

static URANDOM: std::sync::OnceLock<std::fs::File> = std::sync::OnceLock::new();

fn entropy(buf: &mut [u8]) {
    if let Some(f) = URANDOM
        .get_or_init(|| std::fs::File::open("/dev/urandom").ok())
        .and_then(|f| f.try_clone().ok())
    {
        let mut reader = f;
        if reader.read_exact(buf).is_ok() {
            return;
        }
    }
    // Time-seeded fallback (deterministic-ish; only reached without /dev/urandom).
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let mut acc = now ^ (buf.len() as u64).wrapping_mul(0x9e3779b97f4a7c15);
    for b in buf.iter_mut() {
        acc ^= acc << 13;
        acc ^= acc >> 7;
        acc ^= acc << 17;
        *b = (acc & 0xff) as u8;
    }
}

/// Equivalent of `Math.random()` — f64 in `[0, 1)`.
pub(crate) fn random01() -> f64 {
    let mut buf = [0u8; 8];
    entropy(&mut buf);
    let v = u64::from_le_bytes(buf) >> 11;
    (v as f64) / (1u64 << 53) as f64
}

/// Equivalent of `crypto.randomUUID()` — RFC 4122 v4, lowercase hyphenated.
pub(crate) fn uuid_v4() -> String {
    let mut buf = [0u8; 16];
    entropy(&mut buf);
    buf[6] = (buf[6] & 0x0f) | 0x40; // version 4
    buf[8] = (buf[8] & 0x3f) | 0x80; // variant 10
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10],
        buf[11], buf[12], buf[13], buf[14], buf[15]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha1_known_vectors() {
        // node:crypto sha1 of "" / "abc" — must match verbatim.
        assert_eq!(fast_str(""), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_eq!(fast_str("abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn sha256_matches_known_vector() {
        let d = crate::util::encode::hash_sha256_bytes(b"abc");
        assert_eq!(
            hex(&d),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let empty = crate::util::encode::hash_sha256_bytes(b"");
        assert_eq!(
            hex(&empty),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn uuid_version_and_variant() {
        let u = uuid_v4();
        assert_eq!(u.len(), 36);
        assert_eq!(&u[14..15], "4");
        assert!(matches!(&u[19..20], "8" | "9" | "a" | "b"));
    }
}
