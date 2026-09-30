// source: src/snapshot/index.ts — exports: Patch, FileDiff, Interface,
// Service, node, Snapshot
// PROVISIONAL pending crates/core (layer-node, fs-util, process, hash,
// global, project/sql, database) + @/effect/instance-state + @/config/config +
// @opencode-ai/schema/file-diff + diff npm: consts + pure helpers
// (parseWorktree-style list parsing, op batching clash rule, cat-file batch
// header validation messages, revert batching) ported verbatim; git/process
// bodies as trait.

use serde::{Deserialize, Serialize};

/// source: Patch { hash, files } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub hash: String,
    pub files: Vec<String>,
}

/// source: FileDiff = Info (file-diff schema) — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub file: String,
    pub patch: String,
    pub additions: i64,
    pub deletions: i64,
    pub status: String,
}

/// source: prune = "7.days" — verbatim.
pub const PRUNE: &str = "7.days";
/// source: limit = 2 * 1024 * 1024 — verbatim.
pub const LIMIT: u64 = 2 * 1024 * 1024;
/// source: core = ["-c", "core.longpaths=true", "-c", "core.symlinks=true"] — verbatim.
pub const CORE: &[&str] = &["-c", "core.longpaths=true", "-c", "core.symlinks=true"];
/// source: cfg = ["-c", "core.autocrlf=false", ...core] — verbatim.
pub const CFG: &[&str] = &[
    "-c",
    "core.autocrlf=false",
    "-c",
    "core.longpaths=true",
    "-c",
    "core.symlinks=true",
];
/// source: quote = [...cfg, "-c", "core.quotepath=false"] — verbatim.
pub const QUOTE: &[&str] = &[
    "-c",
    "core.autocrlf=false",
    "-c",
    "core.longpaths=true",
    "-c",
    "core.symlinks=true",
    "-c",
    "core.quotepath=false",
];

/// source: revert batch size 100 — verbatim.
pub const REVERT_BATCH: usize = 100;
/// source: diffFull paging step 100 — verbatim.
pub const DIFF_STEP: usize = 100;

/// source: Interface — init/cleanup/track/patch/restore/revert/diff/diffFull, verbatim.
pub trait Interface {
    fn init(&self);
    fn cleanup(&self);
    fn track(&self) -> Option<String>;
    fn patch(&self, hash: &str) -> Patch;
    fn restore(&self, snapshot: &str);
    fn revert(&self, patches: &[Patch]);
    fn diff(&self, hash: &str) -> String;
    fn diff_full(&self, from: &str, to: &str) -> Vec<FileDiff>;
}

/// source: Service "@opencode/Snapshot" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Snapshot";

/// source: node deps [FSUtil.node, AppProcess.node, Config.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/fs-util.FSUtil",
    "@opencode-ai/core/process.AppProcess",
    "@/config/config.Config",
];

/// source: cat-file --batch fallback messages — verbatim.
pub const CATFILE_FALLBACK_LOG: &str =
    "git cat-file --batch failed during snapshot diff, falling back to per-file git show";
pub const CATFILE_TRUNCATED: &str =
    "git cat-file --batch returned a truncated header during snapshot diff, falling back to per-file git show";
pub const CATFILE_BAD_HEADER: &str =
    "git cat-file --batch returned an unexpected header during snapshot diff, falling back to per-file git show";
pub const CATFILE_TRUNC_CONTENT: &str =
    "git cat-file --batch returned truncated content during snapshot diff, falling back to per-file git show";
pub const CATFILE_TRAILING: &str =
    "git cat-file --batch returned trailing data during snapshot diff, falling back to per-file git show";

/// source: snapshot log strings — verbatim.
pub const LOG_INIT: &str = "initialized";
pub const LOG_CLEANUP: &str = "cleanup";
pub const LOG_CLEANUP_FAILED: &str = "cleanup failed";
pub const LOG_LIST_FAILED: &str = "failed to list snapshot files";
pub const LOG_ADD_FAILED: &str = "failed to add snapshot files";
pub const LOG_DIFF_FAILED: &str = "failed to get diff";
pub const LOG_REVERTING: &str = "reverting";
pub const LOG_RESTORE: &str = "restore";
pub const LOG_RESTORE_FAILED: &str = "failed to restore snapshot";

/// source: revert clash() — verbatim path-overlap rule.
pub fn clash(a: &str, b: &str) -> bool {
    a == b || a.starts_with(&format!("{}/", b)) || b.starts_with(&format!("{}/", a))
}

/// source: status code mapping — A→added, D→deleted, else modified. Verbatim.
pub fn status_of(code: &str) -> &'static str {
    if code.starts_with('A') {
        "added"
    } else if code.starts_with('D') {
        "deleted"
    } else {
        "modified"
    }
}

/// source: numstat row parse — "-" → binary/0, parseInt with finite guard. Verbatim.
pub fn parse_numstat(adds: &str, dels: &str) -> (bool, i64, i64) {
    let binary = adds == "-" && dels == "-";
    let additions = if binary {
        0
    } else {
        adds.parse::<i64>().unwrap_or(0).max(0)
    };
    let deletions = if binary {
        0
    } else {
        dels.parse::<i64>().unwrap_or(0).max(0)
    };
    (binary, additions, deletions)
}
