//! Rust port of `packages/app/src/constants/file-picker.ts` (opencode v1.18.30).
//!
//! Source 89 lines. Exports: `ACCEPTED_IMAGE_TYPES`, `ACCEPTED_FILE_TYPES`, `ACCEPTED_FILE_EXTENSIONS`, `filePickerFilters`.
//!
//! 1:1 notes:
//! - `MIME_EXT` and `TEXT_EXT` are module-private in the source; kept private here.
//! - `ACCEPTED_FILE_EXTENSIONS` is a computed module-level `const` in the source
//!   (flatMap → `Set` → `sort`); exposed here as `accepted_file_extensions()`
//!   with identical computation and ordering.
//! - Rename log: `filePickerFilters` → `file_picker_filters`.
//! - Original file: `packages/app/src/constants/file-picker.ts`

#![allow(dead_code)]

pub const ACCEPTED_IMAGE_TYPES: &[&str] = &["image/png", "image/jpeg", "image/gif", "image/webp"];

pub const ACCEPTED_FILE_TYPES: &[&str] = &[
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
    "application/pdf",
    "text/*",
    "application/json",
    "application/ld+json",
    "application/toml",
    "application/x-toml",
    "application/x-yaml",
    "application/xml",
    "application/yaml",
    ".c",
    ".cc",
    ".cjs",
    ".conf",
    ".cpp",
    ".css",
    ".csv",
    ".cts",
    ".env",
    ".go",
    ".gql",
    ".graphql",
    ".h",
    ".hh",
    ".hpp",
    ".htm",
    ".html",
    ".ini",
    ".java",
    ".js",
    ".json",
    ".jsx",
    ".log",
    ".md",
    ".mdx",
    ".mjs",
    ".mts",
    ".py",
    ".rb",
    ".rs",
    ".sass",
    ".scss",
    ".sh",
    ".sql",
    ".toml",
    ".ts",
    ".tsx",
    ".txt",
    ".xml",
    ".yaml",
    ".yml",
    ".zsh",
];

const MIME_EXT: &[(&str, &str)] = &[
    ("image/png", "png"),
    ("image/jpeg", "jpg"),
    ("image/gif", "gif"),
    ("image/webp", "webp"),
    ("application/pdf", "pdf"),
    ("application/json", "json"),
    ("application/ld+json", "jsonld"),
    ("application/toml", "toml"),
    ("application/x-toml", "toml"),
    ("application/x-yaml", "yaml"),
    ("application/xml", "xml"),
    ("application/yaml", "yaml"),
];

const TEXT_EXT: &[&str] = &["txt", "text", "md", "markdown", "log", "csv"];

/// Mirrors the computed `ACCEPTED_FILE_EXTENSIONS` export:
/// `Array.from(new Set(ACCEPTED_FILE_TYPES.flatMap(...))).sort()`.
pub fn accepted_file_extensions() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for item in ACCEPTED_FILE_TYPES {
        if let Some(rest) = item.strip_prefix('.') {
            out.push(rest);
        } else if *item == "text/*" {
            out.extend_from_slice(TEXT_EXT);
        } else if let Some(ext) = MIME_EXT.iter().find(|(mime, _)| *mime == *item) {
            out.push(ext.1);
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Mirrors `filePickerFilters`.
#[derive(Debug, Clone, PartialEq)]
pub struct FilePickerFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// Mirrors `filePickerFilters(name, ext?)`.
pub fn file_picker_filters(name: &str, ext: Option<Vec<String>>) -> Option<Vec<FilePickerFilter>> {
    let ext = ext?;
    if ext.is_empty() {
        return None;
    }
    Some(vec![FilePickerFilter {
        name: name.to_string(),
        extensions: ext,
    }])
}
