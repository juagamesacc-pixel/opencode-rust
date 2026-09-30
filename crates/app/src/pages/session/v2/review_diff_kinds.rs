//! Rust port of `packages/app/src/pages/session/v2/review-diff-kinds.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/v2/review-diff-kinds.ts` -> `session/v2/review_diff_kinds.rs` (kebab -> snake_case).

use std::collections::HashMap;

pub fn normalize_path(p: &str) -> String {
    p.replace('\\', "/")
}

pub fn filter_renderable_diff(file: Option<&str>) -> bool {
    file.is_some()
}

pub fn review_diff_needs_load(additions: i32, deletions: i32, patch: Option<&str>) -> bool {
    if additions == 0 && deletions == 0 {
        return false;
    }
    match patch {
        None => true,
        Some(p) => !p.contains("@@ "),
    }
}

pub fn review_root_directory(root: &str) -> String {
    if root == "/" || root.len() <= 3 && root.chars().nth(1) == Some(':') {
        return root.to_string();
    }
    root.trim_end_matches(['/', '\\']).to_string()
}

pub fn filter_review_files(files: &[String], query: &str) -> Vec<String> {
    let v = query.trim().to_lowercase();
    if v.is_empty() {
        return files.to_vec();
    }
    files
        .iter()
        .filter(|f| f.to_lowercase().contains(&v))
        .cloned()
        .collect()
}

pub fn review_diff_kinds(diffs: &[(String, String)]) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    for (file, status) in diffs {
        let f = normalize_path(file);
        let kind = match status.as_str() {
            "added" => "add",
            "deleted" => "del",
            _ => "mix",
        };
        out.insert(f.clone(), kind.to_string());
        let parts: Vec<&str> = f.split('/').collect();
        for idx in 0..parts.len() - 1 {
            let dir = parts[..=idx].join("/");
            if dir.is_empty() {
                continue;
            }
            let existing = out.get(&dir).cloned();
            let merged = match existing.as_deref() {
                None => kind.to_string(),
                Some(a) if a == kind => a.to_string(),
                _ => "mix".to_string(),
            };
            out.insert(dir, merged);
        }
    }
    out
}
