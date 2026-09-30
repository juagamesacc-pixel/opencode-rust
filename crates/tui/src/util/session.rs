// source: packages/tui/src/util/session.ts (3 lines, v1.18.30)
// 1:1 port — default-title regex without a regex crate: the prefix
// forms are `New session - ` / `Child session - ` followed by an
// ISO timestamp with milliseconds.

#![allow(dead_code)]

/// Mirrors `isDefaultTitle`.
pub fn is_default_title(title: &str) -> bool {
    for prefix in ["New session - ", "Child session - "] {
        if let Some(rest) = title.strip_prefix(prefix) {
            return is_iso_millis(rest);
        }
    }
    false
}

fn is_iso_millis(value: &str) -> bool {
    // `YYYY-MM-DDTHH:MM:SS.mmmZ`
    let bytes = value.as_bytes();
    if bytes.len() != 24 {
        return false;
    }
    let digits =
        |range: std::ops::Range<usize>| range.into_iter().all(|i| bytes[i].is_ascii_digit());
    digits(0..4)
        && bytes[4] == b'-'
        && digits(5..7)
        && bytes[7] == b'-'
        && digits(8..10)
        && bytes[10] == b'T'
        && digits(11..13)
        && bytes[13] == b':'
        && digits(14..16)
        && bytes[16] == b':'
        && digits(17..19)
        && bytes[19] == b'.'
        && digits(20..23)
        && bytes[23] == b'Z'
}
