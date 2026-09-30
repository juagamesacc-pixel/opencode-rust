// source: src/util/encode.ts — exports: base64Encode, base64Decode, hash,
// checksum, sampledChecksum
//
// PROVISIONAL pending node:crypto / WebCrypto equivalents:
// - `btoa`/`atob` and `TextEncoder`/`TextDecoder` are re-implemented in pure
//   Rust below (URL-safe base64, UTF-8 with lossy decode — U+FFFD for invalid
//   sequences, matching TextDecoder's default replacement).
// - `crypto.subtle.digest("SHA-256")` -> pure-Rust SHA-256 (no deps; only the
//   `SHA-256` default algorithm is supported — the sole value used in source).

/// source: base64Encode(value) — UTF-8 -> base64, then `+/` -> `-_` and strip
/// `=`. Verbatim (no padding ever emitted).
pub fn base64_encode(value: &str) -> String {
    const B64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = value.as_bytes();
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64[(n >> 18) as usize & 63] as char);
        out.push(B64[(n >> 12) as usize & 63] as char);
        if chunk.len() > 1 {
            out.push(B64[(n >> 6) as usize & 63] as char);
        }
        if chunk.len() > 2 {
            out.push(B64[n as usize & 63] as char);
        }
    }
    out.replace('+', "-").replace('/', "_")
}

/// source: base64Decode(value) — `-_` -> `+/`, then `atob`, then UTF-8 decode.
/// `atob` throws `InvalidCharacterError` with this exact message on bad input;
/// the decoded bytes are then lossy-decoded (U+FFFD replacement).
pub fn base64_decode(value: &str) -> Result<String, String> {
    const INVALID: &str = "The string to be decoded is not correctly encoded.";
    let cleaned: String = value
        .chars()
        .filter(|c| !matches!(c, ' ' | '\t' | '\n' | '\r' | '\u{000C}'))
        .collect();
    if cleaned.len() % 4 != 0 {
        return Err(INVALID.to_string());
    }

    fn val(c: u8, padding: bool) -> Result<u32, String> {
        match c {
            b'A'..=b'Z' => Ok((c - b'A') as u32),
            b'a'..=b'z' => Ok((c - b'a' + 26) as u32),
            b'0'..=b'9' => Ok((c - b'0' + 52) as u32),
            b'+' => Ok(62),
            b'/' => Ok(63),
            b'=' => {
                if padding {
                    Ok(0)
                } else {
                    Err("The string to be decoded is not correctly encoded.".to_string())
                }
            }
            _ => Err("The string to be decoded is not correctly encoded.".to_string()),
        }
    }

    let bytes = cleaned.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for group in bytes.chunks_exact(4) {
        let mut vals = [0u32; 4];
        let mut pad = 0;
        for (i, &c) in group.iter().enumerate() {
            if c == b'=' {
                pad += 1;
                if pad > 2 || i < 2 {
                    return Err(INVALID.to_string());
                }
            }
            vals[i] = val(c, c == b'=')?;
        }
        let n = (vals[0] << 18) | (vals[1] << 12) | (vals[2] << 6) | vals[3];
        out.push((n >> 16) as u8);
        if group[2] != b'=' {
            out.push((n >> 8) as u8);
        }
        if group[3] != b'=' {
            out.push(n as u8);
        }
    }

    Ok(String::from_utf8_lossy(&out).into_owned())
}

/// source: hash(content, algorithm = "SHA-256") via `crypto.subtle.digest`.
/// Hex-encoded digest. Only SHA-256 is implemented (the default; no other
/// caller passes an algorithm) — PROVISIONAL note above.
pub fn hash(content: &str, algorithm: Option<&str>) -> String {
    debug_assert!(matches!(algorithm, None | Some("SHA-256")));
    sha256_hex(content.as_bytes())
}

/// Byte digest of the SHA-256 primitive (shared with `util/hash::sha256`).
pub(crate) fn hash_sha256_bytes(data: &[u8]) -> [u8; 32] {
    sha256_bytes(data)
}

/// source: checksum(content) — FNV-1a 32-bit over UTF-16 code units
/// (`charCodeAt` semantics), `Math.imul` (wrapping 32-bit multiply),
/// `>>> 0` (uint32), then `toString(36)` (lowercase base36). Empty -> None.
pub fn checksum(content: &str) -> Option<String> {
    if content.is_empty() {
        return None;
    }
    let units: Vec<u16> = content.encode_utf16().collect();
    Some(to_base36(fnv1a(&units)))
}

/// source: sampledChecksum(content, limit = 500_000) — for content longer
/// than `limit`, checksums 5 windows of 4096 UTF-16 units anchored at
/// 0/25%/50%/75%/end (window centered on the point, clamped), joined with
/// `:`, prefixed with `content.length:`. Verbatim.
pub fn sampled_checksum(content: &str, limit: Option<usize>) -> Option<String> {
    let limit = limit.unwrap_or(500_000);
    if content.is_empty() {
        return None;
    }
    let units: Vec<u16> = content.encode_utf16().collect();
    let len = units.len();
    if len <= limit {
        return checksum(content);
    }

    let size = 4096usize;
    let points: [usize; 5] = [
        0,
        len * 25 / 100,
        len * 50 / 100,
        len * 75 / 100,
        len.saturating_sub(size),
    ];
    let hashes: Vec<String> = points
        .iter()
        .map(|&point| {
            // Math.max(0, Math.min(len - size, point - Math.floor(size / 2)))
            let start = (len.saturating_sub(size)).min(point.saturating_sub(size / 2));
            let end = (start + size).min(len);
            to_base36(fnv1a(&units[start..end]))
        })
        .collect();
    Some(format!("{len}:{}", hashes.join(":")))
}

// ---------------------------------------------------------------------------
// Pure-Rust FNV-1a / base36 (see header note)
// ---------------------------------------------------------------------------

fn fnv1a(units: &[u16]) -> u32 {
    let mut hash: u32 = 0x811c_9dc5;
    for &u in units {
        hash ^= u as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

fn to_base36(mut v: u32) -> String {
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    if v == 0 {
        return "0".to_string();
    }
    let mut out = Vec::new();
    while v > 0 {
        out.push(DIGITS[(v % 36) as usize]);
        v /= 36;
    }
    out.reverse();
    // SAFETY: DIGITS is ASCII.
    String::from_utf8(out).unwrap()
}

// ---------------------------------------------------------------------------
// Pure-Rust SHA-256 (WebCrypto `crypto.subtle.digest` equivalent)
// ---------------------------------------------------------------------------

const K256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn sha256_hex(data: &[u8]) -> String {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for block in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K256[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = String::with_capacity(64);
    for v in h {
        out.push_str(&format!("{v:08x}"));
    }
    out
}
