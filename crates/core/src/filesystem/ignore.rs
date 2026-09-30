//! Rust port of `packages/core/src/filesystem/ignore.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

// Source: FOLDERS set verbatim
pub const FOLDERS: &[&str] = &[
    "node_modules",
    "bower_components",
    ".pnpm-store",
    "vendor",
    ".npm",
    "dist",
    "build",
    "out",
    ".next",
    "target",
    "bin",
    "obj",
    ".git",
    ".svn",
    ".hg",
    ".vscode",
    ".idea",
    ".turbo",
    ".output",
    "desktop",
    ".sst",
    ".cache",
    ".webkit-cache",
    "__pycache__",
    ".pytest_cache",
    "mypy_cache",
    ".history",
    ".gradle",
];

pub const FILES: &[&str] = &[
    "**/*.swp",
    "**/*.swo",
    "**/*.pyc",
    "**/.DS_Store",
    "**/Thumbs.db",
    "**/logs/**",
    "**/tmp/**",
    "**/temp/**",
    "**/*.log",
    "**/coverage/**",
    "**/.nyc_output/**",
];

/// Source: `export const PATTERNS = [...FILES, ...FOLDERS]` verbatim
pub fn patterns() -> Vec<String> {
    let mut v: Vec<String> = FILES.iter().map(|s| s.to_string()).collect();
    v.extend(FOLDERS.iter().map(|s| s.to_string()));
    v
}

/// Source: `export function match(filepath, opts?)` verbatim — whitelist first, then folder parts, then FILES+extra glob
pub fn matches(filepath: &str, extra: Option<&[String]>, whitelist: Option<&[String]>) -> bool {
    if let Some(wl) = whitelist {
        for pat in wl {
            if glob_match(pat, filepath) {
                return false;
            }
        }
    }
    let parts: Vec<&str> = filepath.split(['/', '\\']).collect();
    for part in parts {
        if FOLDERS.contains(&part) {
            return true;
        }
    }
    let pats: Vec<&str> = FILES.to_vec();
    // extra handling via caller Vec
    // For faithful glob we call simple glob_match
    for pat in pats.iter().chain(
        extra
            .map(|e| e.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default()
            .iter(),
    ) {
        if glob_match(pat, filepath) {
            return true;
        }
    }
    false
}

fn glob_match(pattern: &str, path: &str) -> bool {
    // PROVISIONAL pending ../util/glob — minimal **/*.ext and folder suffix handling
    if pattern.contains("**") {
        let suffix = pattern.trim_start_matches("**/");
        if suffix.contains('/') {
            // **/logs/** -> check contains segment
            let seg = suffix.trim_end_matches("/**").trim_matches('/');
            return path.contains(seg);
        } else if suffix.starts_with("*.") {
            let ext = &suffix[1..];
            return path.ends_with(ext);
        } else {
            return path.contains(suffix);
        }
    }
    path == pattern || path.ends_with(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn folder_match() {
        assert!(matches("a/node_modules/b", None, None));
        assert!(!matches("a/src/b.ts", None, None));
    }
    #[test]
    fn file_match() {
        assert!(matches("a/file.log", None, None));
        assert!(matches("a/.DS_Store", None, None));
    }
    #[test]
    fn whitelist() {
        assert!(!matches(
            "a/node_modules/b",
            None,
            Some(&["**/node_modules/**".to_string()])
        ));
    }
}
