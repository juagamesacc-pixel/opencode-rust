//! Rust port of `packages/core/src/database/path.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

/// Source: `function storagePath(input: string)` — verbatim win32 slash handling
pub fn storage_path(input: &str, is_win32: bool) -> String {
    if !is_win32 {
        return input.to_string();
    }
    input.replace('\\', "/")
}

pub fn is_windows_storage_path(input: &str) -> bool {
    // /^[A-Za-z]:\// or starts with //
    let bytes = input.as_bytes();
    (bytes.len() >= 3 && bytes[1] == b':' && bytes[2] == b'/' && bytes[0].is_ascii_alphabetic())
        || input.starts_with("//")
}

pub fn absolute(input: &str, is_win32: bool) -> Result<String, String> {
    let result = storage_path(input, is_win32);
    let is_abs = result.starts_with('/') || (is_win32 && is_windows_storage_path(&result));
    if !is_abs {
        return Err(format!("Path is not absolute: {}", input));
    }
    Ok(result)
}

pub fn to_platform(input: &str, is_win32: bool) -> String {
    if !is_win32 || !is_windows_storage_path(input) {
        return input.to_string();
    }
    input.replace('/', "\\")
}

// Legacy empty directory handling preserved: `input ? absolute(input) : input`
pub fn directory_to_driver(input: &str, is_win32: bool) -> Result<String, String> {
    if input.is_empty() {
        return Ok(String::new());
    }
    absolute(input, is_win32)
}
pub fn directory_from_driver(input: &str, is_win32: bool) -> Result<String, String> {
    if input.is_empty() {
        return Ok(String::new());
    }
    let abs = absolute(input, is_win32)?;
    Ok(to_platform(&abs, is_win32))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn storage_win() {
        assert_eq!(storage_path("a\\b", true), "a/b");
        assert_eq!(storage_path("a/b", false), "a/b");
    }
    #[test]
    fn absolute_err() {
        assert!(absolute("relative", false).is_err());
        assert!(absolute("/abs", false).is_ok());
    }
    #[test]
    fn windows_abs() {
        assert!(is_windows_storage_path("C:/foo"));
        assert!(is_windows_storage_path("//server/share"));
        assert!(!is_windows_storage_path("/unix"));
    }
    #[test]
    fn empty_dir() {
        assert_eq!(directory_to_driver("", false).unwrap(), "");
        assert_eq!(directory_from_driver("", false).unwrap(), "");
    }
}
