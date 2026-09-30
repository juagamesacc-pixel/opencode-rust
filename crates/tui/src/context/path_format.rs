// source: packages/tui/src/context/path-format.tsx (24 lines, v1.18.30)
// 1:1 port — pure functions over explicit inputs.

#![allow(dead_code)]

use crate::runtime::{abbreviate_home, relative_path};

/// Mirrors the `path()` accessor — location directory or cwd.
pub fn base_path(location_directory: Option<&str>, cwd: &str) -> String {
    match location_directory {
        Some(dir) if !dir.is_empty() => dir.to_string(),
        _ => cwd.to_string(),
    }
}

/// Mirrors `format`/`formatPath` — relative when inside base, `"."` for
/// the base itself, abbreviated absolute otherwise.
pub fn format_path(input: Option<&str>, base: &str, home: &str) -> String {
    let input = match input {
        Some(input) if !input.is_empty() => input,
        _ => return String::new(),
    };
    let absolute =
        if input.starts_with('/') || input.starts_with('\\') || input.get(1..2) == Some(":") {
            input.to_string()
        } else {
            join_base(base, input)
        };
    let relative = relative_path(base, &absolute);
    if relative.is_empty() {
        return ".".to_string();
    }
    if relative != ".." && !relative.starts_with("../") {
        return relative;
    }
    abbreviate_home(&absolute, home)
}

fn join_base(base: &str, input: &str) -> String {
    let base = base.trim_end_matches('/');
    format!("{base}/{input}")
}
