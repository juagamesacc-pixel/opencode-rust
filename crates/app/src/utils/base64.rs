//! Rust port of `packages/app/src/utils/base64.ts` (opencode v1.18.30) plus the
//! shared std-only base64 codec backing `btoa` / `@opencode-ai/core/util/encode`.
//!
//! Source 10 lines: `decode64(value)` wraps `base64Decode` from
//! `@opencode-ai/core/util/encode` in try/catch; `undefined` input or a decode
//! error returns `undefined`.
//!
//! 1:1 notes:
//! - `decode64` is the only export of the source file. The codec helpers below
//!   (`encode_padded` = `btoa`, `encode_padless` = the `base64Encode` used by
//!   `session-route.ts` before it strips `=` padding) are internal to the lane
//!   and stand in for the unavailable `@opencode-ai/core` runtime.
//! - `base64Decode` behaviour is modelled as a strict, padding-tolerant decoder
//!   (rejects non-alphabet characters such as spaces or `-`).
//! - Original file: `packages/app/src/utils/base64.ts`

#![allow(dead_code)]

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Mirrors `decode64(value: string | undefined)`.
/// Returns `None` for `None` input or any decode/UTF-8 failure.
pub fn decode64(value: Option<&str>) -> Option<String> {
    let value = value?;
    let bytes = decode(value).ok()?;
    String::from_utf8(bytes).ok()
}

/// `btoa` equivalent: standard base64 with `=` padding.
pub fn encode_padded(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < bytes.len() {
        let a = bytes[i];
        let b = bytes.get(i + 1).copied().unwrap_or(0);
        let c = bytes.get(i + 2).copied().unwrap_or(0);
        out.push(ALPHABET[(a >> 2) as usize] as char);
        out.push(ALPHABET[(((a & 0x03) << 4) | (b >> 4)) as usize] as char);
        let pad2 = i + 1 >= bytes.len();
        out.push(if pad2 {
            '='
        } else {
            ALPHABET[(((b & 0x0F) << 2) | (c >> 6)) as usize] as char
        });
        let pad3 = i + 2 >= bytes.len();
        out.push(if pad3 {
            '='
        } else {
            ALPHABET[(c & 0x3F) as usize] as char
        });
        i += 3;
    }
    out
}

/// `base64Encode` equivalent: standard base64 with trailing `=` padding stripped
/// (the form used by `sessionHref`/`legacySessionHref`).
pub fn encode_padless(input: &str) -> String {
    encode_padded(input).trim_end_matches('=').to_string()
}

fn decode(input: &str) -> Result<Vec<u8>, ()> {
    let bytes = input.as_bytes();
    let mut end = bytes.len();
    while end > 0 && bytes[end - 1] == b'=' {
        end -= 1;
    }
    let body = &bytes[..end];
    if body.len() % 4 == 1 {
        return Err(());
    }
    let mut out = Vec::with_capacity((body.len() / 4) * 3 + 2);
    let mut i = 0;
    while i < body.len() {
        let mut sextets = [0u8; 4];
        let mut n = 0;
        while n < 4 && i < body.len() {
            sextets[n] = value(body[i]).ok_or(())?;
            n += 1;
            i += 1;
        }
        if n == 1 {
            return Err(());
        }
        out.push((sextets[0] << 2) | (sextets[1] >> 4));
        if n >= 3 {
            out.push(((sextets[1] & 0x0F) << 4) | (sextets[2] >> 2));
        }
        if n == 4 {
            out.push(((sextets[2] & 0x03) << 6) | sextets[3]);
        }
    }
    Ok(out)
}

fn value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}
