// source: packages/tui/src/util/revert-diff.ts (18 lines, v1.18.30)
// 1:1 port — patch parsing over unified diffs; filename preference
// (new over old, `/dev/null` skipped, `a/`/`b/` prefix stripped) and
// per-file additions/deletions counts verbatim. Parse failures → `[]`.

#![allow(dead_code)]

/// One file entry (mirrors the mapped patch shape).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevertDiffFile {
    pub filename: String,
    pub additions: usize,
    pub deletions: usize,
}

/// Mirrors `getRevertDiffFiles`.
pub fn get_revert_diff_files(diff_text: &str) -> Vec<RevertDiffFile> {
    if diff_text.is_empty() {
        return Vec::new();
    }
    let mut files: Vec<RevertDiffFile> = Vec::new();
    let mut current: Option<RevertDiffFile> = None;
    for line in diff_text.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            if let Some(file) = current.take() {
                files.push(file);
            }
            current = Some(new_file(rest));
            continue;
        }
        if line.starts_with("+++ ") || line.starts_with("--- ") {
            if let Some(file) = current.as_mut() {
                let candidate = line[4..].split('\t').next().unwrap_or("").trim().to_string();
                if candidate != "/dev/null" && !file.filename.is_empty() && file.filename == "unknown" {
                    file.filename = strip_prefix(&candidate);
                }
            }
            continue;
        }
        if line.starts_with("+++ ") || line.starts_with("--- ") || line.starts_with("@@") {
            continue;
        }
        if let Some(file) = current.as_mut() {
            if line.starts_with('+') && !line.starts_with("+++") {
                file.additions += 1;
            } else if line.starts_with('-') && !line.starts_with("---") {
                file.deletions += 1;
            }
        }
    }
    if let Some(file) = current {
        files.push(file);
    }
    files
}

fn new_file(git_header: &str) -> RevertDiffFile {
    // `a/path b/path` — prefer the b-side, fall back to the a-side.
    let parts: Vec<&str> = git_header.split_whitespace().collect();
    let candidate = parts.iter().rev().find(|part| *part != "/dev/null").copied().unwrap_or("unknown");
    RevertDiffFile { filename: strip_prefix(candidate), additions: 0, deletions: 0 }
}

fn strip_prefix(value: &str) -> String {
    value.strip_prefix("a/").or_else(|| value.strip_prefix("b/")).unwrap_or(value).to_string()
}