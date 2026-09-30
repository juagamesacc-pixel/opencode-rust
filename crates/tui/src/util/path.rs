// source: packages/tui/src/util/path.ts (12 lines, v1.18.30)
// 1:1 port — win32-only normalization; other platforms pass through
// verbatim, and a failing `realpath` falls back to the normalized path.

#![allow(dead_code)]

/// Mirrors `normalizePath`.
pub fn normalize_path(input: &str, platform: &str) -> String {
    if platform != "win32" {
        return input.to_string();
    }
    let slashed = input.replace('/', "\\");
    let resolved = win32_normalize(&win32_resolve(&slashed));
    std::fs::canonicalize(&resolved)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or(resolved)
}

/// Minimal `win32.resolve` (drive-relative + cwd join).
pub fn win32_resolve(input: &str) -> String {
    if input.starts_with('\\') {
        let cwd = std::env::var("CD")
            .ok()
            .or_else(|| std::env::var("PWD").ok())
            .unwrap_or_else(|| "C:\\".to_string());
        let base = drive_of(&cwd).unwrap_or_else(|| "C:".to_string());
        return win32_normalize(&format!("{base}{input}"));
    }
    win32_normalize(input)
}

fn drive_of(path: &str) -> Option<String> {
    let bytes = path.as_bytes();
    if bytes.len() >= 2 && bytes[1] == b':' {
        Some(path[..2].to_string())
    } else {
        None
    }
}

/// Minimal `win32.normalize` (`.`/`..` folding, backslash separators).
pub fn win32_normalize(input: &str) -> String {
    let drive = drive_of(input).unwrap_or_default();
    let rest = &input[input.len() - (input.len() - drive.len())..];
    let mut parts: Vec<&str> = Vec::new();
    for part in rest.split('\\') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.last().is_some_and(|last| *last != "..") {
                    parts.pop();
                } else if drive.is_empty() {
                    parts.push("..");
                }
            }
            other => parts.push(other),
        }
    }
    let joined = parts.join("\\");
    let normalized = if drive.is_empty() {
        if joined.is_empty() {
            ".".to_string()
        } else {
            joined
        }
    } else {
        format!("{}\\{}", drive, joined)
    };
    normalized
}
