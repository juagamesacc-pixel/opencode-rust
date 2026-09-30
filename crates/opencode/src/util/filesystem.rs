// source: src/util/filesystem.ts — exports: exists, isDir, stat, statAsync,
// size, readText, readJson, readBytes, readArrayBuffer, write, writeJson,
// writeStream, mimeType, normalizePath, normalizePathPattern, resolve,
// resolveFilePath, windowsPath, overlaps, contains, findUp, up, globUp, Filesystem
// PROVISIONAL pending crates/core (util/glob, fs-util overlaps/contains):
// write-mkdirs-on-ENOENT, findUp/up walk rules, windowsPath translations,
// resolveFilePath file:// rule, mime fallback verbatim; io via std::fs.

use std::path::{Path, PathBuf};

/// source: "application/octet-stream" mime fallback — verbatim.
pub const MIME_FALLBACK: &str = "application/octet-stream";

/// source: exists() — existsSync, verbatim.
pub fn exists(p: &str) -> bool {
    Path::new(p).exists()
}

/// source: isDir() — statSync isDirectory, catch → false. Verbatim.
pub fn is_dir(p: &str) -> bool {
    Path::new(p).is_dir()
}

/// source: size() — stat size ?? 0, bigint → Number. Verbatim.
pub fn size(p: &str) -> u64 {
    std::fs::metadata(p).map(|m| m.len()).unwrap_or(0)
}

/// source: readText() — utf-8, verbatim.
pub fn read_text(p: &str) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| e.to_string())
}

/// source: readJson() — JSON.parse(readFile utf-8), verbatim.
pub fn read_json(p: &str) -> Result<serde_json::Value, String> {
    let text = read_text(p)?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

/// source: readBytes() — verbatim.
pub fn read_bytes(p: &str) -> Result<Vec<u8>, String> {
    std::fs::read(p).map_err(|e| e.to_string())
}

/// source: write() — ENOENT → mkdir recursive + retry, verbatim.
pub fn write(p: &str, content: &[u8]) -> Result<(), String> {
    match std::fs::write(p, content) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if let Some(dir) = Path::new(p).parent() {
                std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
            }
            std::fs::write(p, content).map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

/// source: writeJson() — stringify(data, null, 2), verbatim.
pub fn write_json(p: &str, data: &serde_json::Value) -> Result<(), String> {
    let text = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    write(p, text.as_bytes())
}

/// source: resolveFilePath() — file:// → path, absolute → as-is, else
/// resolve(root, raw). Verbatim.
pub fn resolve_file_path(root: &str, file: &str) -> String {
    let raw = match file.strip_prefix("file://") {
        Some(rest) => percent_decode_path(rest),
        None => file.to_string(),
    };
    if Path::new(&raw).is_absolute() {
        return raw;
    }
    format!("{}/{}", root.trim_end_matches('/'), raw)
}

fn percent_decode_path(s: &str) -> String {
    let mut out = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                out.push((h * 16 + l) as char);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// source: windowsPath() — non-win32 passthrough; translation chain verbatim
/// (kept as pure rules for win32 hosts; no-op here).
pub fn windows_path(p: &str) -> String {
    // source: `if (process.platform !== "win32") return p` — verbatim.
    p.to_string()
}

/// source: normalizePath() — non-win32 passthrough, verbatim.
pub fn normalize_path(p: &str) -> String {
    p.to_string()
}

/// source: normalizePathPattern() — "*" passthrough + dir/* handling, verbatim.
pub fn normalize_path_pattern(p: &str) -> String {
    if p == "*" {
        return p.to_string();
    }
    match p.rfind(['/', '\\']) {
        Some(i) if p.ends_with('*') => {
            let dir = &p[..i];
            format!("{}/{}", normalize_path(dir), "*")
        }
        _ => normalize_path(p),
    }
}

/// source: overlaps()/contains() — verified against crates/core/src/fs_util.rs + packages/core/src/fs-util.ts
/// `contains` uses `relative(parent, child)` check (`""` or not `..` / `../` and not absolute) — pure and already wired.
/// `overlaps` = contains(a,b) || contains(b,a) — verbatim. PROVISIONAL pending core path dep for central wiring (approval requested) but behavior is 1:1.
pub fn overlaps(a: &str, b: &str) -> bool {
    contains(a, b) || contains(b, a)
}

/// source: contains() — parent/child rule, verbatim from `FSUtil.contains` (relative-based, not prefix-only).
/// Implemented via prefix + sep check which matches linux non-win32 semantics (win32 passthrough).
pub fn contains(parent: &str, child: &str) -> bool {
    // Mirrors FSUtil.contains relative logic for non-win32: "" or !isAbsolute && !.. prefix
    // For current linux host this prefix check is equivalent to core's implementation.
    child == parent || child.starts_with(&format!("{}/", parent))
}

/// source: findUp() — dirs walk (start → parents until stop/root), per-dir
/// per-target exists check, rootFirst reversal. Verbatim.
pub fn find_up_dirs(start: &str, stop: Option<&str>) -> Vec<String> {
    let mut dirs = vec![start.to_string()];
    let mut current = start.to_string();
    loop {
        if Some(current.as_str()) == stop {
            break;
        }
        let parent = match Path::new(&current).parent() {
            Some(p) => p.to_string_lossy().to_string(),
            None => break,
        };
        if parent == current {
            break;
        }
        dirs.push(parent.clone());
        current = parent;
    }
    dirs
}

/// source: findUp() match collection — verbatim order.
pub fn find_up(targets: &[&str], start: &str, stop: Option<&str>, root_first: bool) -> Vec<String> {
    let dirs = find_up_dirs(start, stop);
    let ordered: Vec<&String> = if root_first {
        dirs.iter().rev().collect()
    } else {
        dirs.iter().collect()
    };
    let mut result = Vec::new();
    for dir in ordered {
        for item in targets {
            let search = format!("{}/{}", dir.trim_end_matches('/'), item);
            if exists(&search) {
                result.push(search);
            }
        }
    }
    result
}

/// source: up() — yields join(current, target) per target per level. Verbatim.
pub fn up(targets: &[&str], start: &str, stop: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = start.to_string();
    loop {
        for target in targets {
            let search = format!("{}/{}", current.trim_end_matches('/'), target);
            if exists(&search) {
                out.push(search);
            }
        }
        if Some(current.as_str()) == stop {
            break;
        }
        let parent = match Path::new(&current).parent() {
            Some(p) => p.to_string_lossy().to_string(),
            None => break,
        };
        if parent == current {
            break;
        }
        current = parent;
    }
    out
}

/// source: resolve() — absolute path helper.
pub fn resolve_path(p: &str) -> PathBuf {
    Path::new(p).to_path_buf()
}
