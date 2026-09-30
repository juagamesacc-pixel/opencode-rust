//! 1:1 port of the `TextEncoder`/`TextDecoder` surface used by
//! `core/src/util/crypto.ts`, `function/src/stat.ts` and
//! `app/src/routes/zen/util/requestBody.ts`.
//!
//! Host runtime shim, not a source file. UTF-8 only, exactly like the JS
//! defaults, and `fatal: false` decoding (U+FFFD replacement) like the default
//! `new TextDecoder()`.

/// `new TextEncoder().encode(text)` — UTF-8 bytes.
pub fn encode(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}

/// `new TextDecoder().decode(bytes)`.
pub fn decode(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// `new TextDecoder().decode(bytes, { stream: true })` — the streaming variant
/// keeps a trailing partial UTF-8 sequence in `pending` instead of emitting a
/// replacement character, so a multi-byte character split across chunk
/// boundaries survives.
pub fn decode_stream(bytes: &[u8], pending: &mut Vec<u8>) -> String {
    pending.extend_from_slice(bytes);
    let text = String::from_utf8_lossy(pending).into_owned();
    // A truncated trailing sequence is held back so the next chunk can complete
    // it, which is why the source passes `stream: true` when splitting bodies.
    let keep = trailing_incomplete_len(pending);
    if keep == 0 {
        pending.clear();
    } else {
        let cut = pending.len() - keep;
        pending.drain(..cut);
    }
    text
}

/// Length of the trailing byte run that cannot yet form a complete UTF-8
/// sequence (1..=3 continuation bytes without a lead, or a truncated lead).
fn trailing_incomplete_len(bytes: &[u8]) -> usize {
    let mut keep = 0usize;
    let mut index = bytes.len();
    while index > 0 {
        let byte = bytes[index - 1];
        if byte & 0b1100_0000 == 0b1000_0000 {
            keep += 1;
            index -= 1;
            if keep == 3 {
                break;
            }
            continue;
        }
        let needed = if byte & 0b1000_0000 == 0 {
            1
        } else if byte & 0b1110_0000 == 0b1100_0000 {
            2
        } else if byte & 0b1111_0000 == 0b1110_0000 {
            3
        } else if byte & 0b1111_1000 == 0b1111_0000 {
            4
        } else {
            1
        };
        if needed > 1 {
            keep += 1;
        }
        break;
    }
    keep
}

/// `encodeURIComponent(text)` re-exported for the cookie writers that live next
/// to the string helpers they share.
pub use super::url::percent_encode;
