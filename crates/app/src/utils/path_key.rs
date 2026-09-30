//! Rust port of `packages/app/src/utils/path-key.ts` (opencode v1.18.30).
//!
//! Source 24 lines: `PathKey`, `pathKey` (verbatim Windows/drive/slash
//! semantics).
//! Original file: `packages/app/src/utils/path-key.ts`

#![allow(dead_code)]

/// Mirrors `PathKey` (branded string).
pub type PathKey = String;

fn is_drive(value: &str) -> bool {
    if value.len() != 2 {
        return false;
    }
    let mut chars = value.chars();
    let first = chars.next().unwrap_or('\0');
    first.is_ascii_alphabetic() && value.as_bytes()[1] == b':'
}

fn trim_trailing_slashes(value: &str) -> &str {
    let bytes = value.as_bytes();
    for i in (0..bytes.len()).rev() {
        if bytes[i] != b'/' {
            return &value[..i + 1];
        }
    }
    ""
}

fn is_windows_path(value: &str) -> bool {
    value.as_bytes().get(1) == Some(&b':') || value.starts_with("\\\\")
}

/// Mirrors `pathKey(path)`.
pub fn path_key(path: &str) -> PathKey {
    let value = if is_windows_path(path) {
        path.replace('\\', "/")
    } else {
        path.to_string()
    };
    let trimmed = trim_trailing_slashes(&value).to_string();
    if trimmed.is_empty() && value.starts_with('/') {
        return "/".to_string();
    }
    if is_drive(&trimmed) {
        return format!("{trimmed}/");
    }
    trimmed
}
