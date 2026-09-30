// source: src/project/vcs.ts — exports: Mode, Event, Info, FileDiff,
// FileStatus, ApplyInput, ApplyResult, PatchApplyError, Interface, Service,
// node, Vcs
// PROVISIONAL pending crates/core (layer-node, filesystem/watcher, event) +
// diff npm + @/*: consts, quoted-path parsing, nums/merge dedupe, status
// sort, apply messages verbatim.

use serde::{Deserialize, Serialize};

/// source: PATCH_CONTEXT_LINES = 2_147_483_647 — verbatim.
pub const PATCH_CONTEXT_LINES: i64 = 2_147_483_647;
/// source: MAX_PATCH_BYTES = 10_000_000 — verbatim.
pub const MAX_PATCH_BYTES: u64 = 10_000_000;
/// source: MAX_TOTAL_PATCH_BYTES = 10_000_000 — verbatim.
pub const MAX_TOTAL_PATCH_BYTES: u64 = 10_000_000;

/// source: Mode = "git" | "branch" — verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Git,
    Branch,
}

/// source: Info { branch?, default_branch? } ("VcsInfo") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
}

/// source: FileDiff ("VcsFileDiff") — patch optional (see #26574), verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
    pub additions: i64,
    pub deletions: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// source: FileStatus — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatus {
    pub file: String,
    pub additions: i64,
    pub deletions: i64,
    pub status: String,
}

/// source: ApplyInput { patch } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyInput {
    pub patch: String,
}

/// source: ApplyResult { applied } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub applied: bool,
}

/// source: PatchApplyError ("VcsPatchApplyError") reason literals — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplyError {
    pub message: String,
    pub reason: String,
}

/// source: "Patch can't be applied because the project is not git-based" — verbatim.
pub const NOT_GIT_MESSAGE: &str = "Patch can't be applied because the project is not git-based";
/// source: "Patch can't be applied" — verbatim.
pub const NOT_CLEAN_MESSAGE: &str = "Patch can't be applied";
pub const REASON_NON_GIT: &str = "non-git";
pub const REASON_NOT_CLEAN: &str = "not-clean";

/// source: parseQuotedPath — C-style escapes \t \n \r \" \\, verbatim.
pub fn parse_quoted_path(value: &str) -> Option<(String, usize)> {
    let bytes = value.as_bytes();
    if bytes.first() != Some(&b'"') {
        return None;
    }
    let mut out = String::new();
    let mut idx = 1;
    while idx < value.len() {
        let c = bytes[idx] as char;
        if c == '"' {
            return Some((out, idx + 1));
        }
        if c != '\\' {
            out.push(c);
            idx += 1;
            continue;
        }
        idx += 1;
        match bytes.get(idx) {
            Some(b't') => out.push('\t'),
            Some(b'n') => out.push('\n'),
            Some(b'r') => out.push('\r'),
            Some(b'"') => out.push('"'),
            Some(b'\\') => out.push('\\'),
            Some(b) => out.push(*b as char),
            None => {}
        }
        idx += 1;
    }
    None
}

/// source: parsePathToken — quoted ? parseQuoted : split("\t")[0]. Verbatim.
pub fn parse_path_token(value: &str) -> String {
    if !value.starts_with('"') {
        return value.split('\t').next().unwrap_or(value).to_string();
    }
    parse_quoted_path(value)
        .map(|(v, _)| v)
        .unwrap_or_else(|| value.to_string())
}

/// source: fileFromDiffPath — /dev/null → undefined; strip a|b/ prefix. Verbatim.
pub fn file_from_diff_path(value: Option<&str>) -> Option<String> {
    let v = value?;
    if v == "/dev/null" {
        return None;
    }
    let file = parse_path_token(v);
    if let Some(rest) = file.strip_prefix("a/").or_else(|| file.strip_prefix("b/")) {
        return Some(rest.to_string());
    }
    Some(file)
}

/// source: nums() — file → { additions, deletions } map. Verbatim.
pub fn nums_map(stats: &[(String, i64, i64)]) -> std::collections::HashMap<String, (i64, i64)> {
    stats
        .iter()
        .map(|(f, a, d)| (f.clone(), (*a, *d)))
        .collect()
}

/// source: merge() — first-wins dedupe by file. Verbatim.
pub fn merge_items(lists: Vec<Vec<(String, String)>>) -> Vec<(String, String)> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for (file, code) in lists.into_iter().flatten() {
        if seen.insert(file.clone()) {
            out.push((file, code));
        }
    }
    out
}

/// source: status() sort — file.localeCompare. Verbatim.
pub fn sort_by_file(files: &mut [String]) {
    files.sort();
}

/// source: Interface — init/branch/defaultBranch/status/diff/diffRaw/apply, verbatim.
pub trait Interface {
    fn branch(&self) -> Option<String>;
    fn default_branch(&self) -> Option<String>;
    fn status(&self) -> Vec<FileStatus>;
    fn diff_raw(&self) -> String;
}

/// source: Service "@opencode/Vcs" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Vcs";

/// source: node deps [Git.node, EventV2Bridge.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@/git.Git", "@/event-v2-bridge.EventV2Bridge"];
