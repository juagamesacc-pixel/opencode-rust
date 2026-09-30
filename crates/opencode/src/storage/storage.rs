// source: src/storage/storage.ts — exports: NotFoundError, Error,
// Interface, Service, node, Storage
// PROVISIONAL pending crates/core (layer-node, global, fs-util, schema) +
// @/git + @/effect/instance-state: file() key-join, missing() ENOENT/NotFound
// rule, parseMigration NaN→0, MIGRATIONS count 2 + glob patterns, list()
// "**/*" + strip-".json" + localeCompare sort, verbatim messages.

use serde::{Deserialize, Serialize};

/// source: NotFoundError ("NotFoundError" { message }) — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundError {
    pub message: String,
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotFoundError: {}", self.message)
    }
}

impl std::error::Error for NotFoundError {}

/// source: fail target — `Resource not found: ${target}`, verbatim.
pub fn not_found_message(target: &str) -> String {
    format!("Resource not found: {}", target)
}

/// source: file(dir, key) — path.join(dir, ...key) + ".json", verbatim.
pub fn file(dir: &str, key: &[&str]) -> String {
    format!("{}/{}.json", dir, key.join("/"))
}

/// source: parseMigration — parseInt base 10, NaN → 0. Verbatim.
pub fn parse_migration(text: &str) -> i64 {
    text.trim().parse::<i64>().unwrap_or(0)
}

/// source: MIGRATIONS.length = 2 — verbatim count.
pub const MIGRATIONS_LEN: usize = 2;
/// source: migration marker filename — verbatim.
pub const MIGRATION_MARKER: &str = "migration";
/// source: storage subdir — verbatim.
pub const STORAGE_DIR: &str = "storage";

/// source: migration.1 globs — verbatim patterns.
pub const MIG1_MSG_GLOB: &str = "storage/session/message/*/*.json";
pub const MIG1_SESSION_GLOB: &str = "storage/session/info/*.json";
pub const MIG1_PART_GLOB_PREFIX: &str = "storage/session/part/";
/// source: migration.2 glob — verbatim.
pub const MIG2_SESSION_GLOB: &str = "session/*/*.json";
/// source: migration.2 session_diff name — verbatim.
pub const SESSION_DIFF_DIR: &str = "session_diff";

/// source: migrating log strings — verbatim templates.
pub const LOG_MIGRATING_PROJECT: &str = "migrating project";
pub const LOG_MIGRATING_SESSIONS: &str = "migrating sessions for project";
pub const LOG_MIGRATING_MESSAGES: &str = "migrating messages for session";
pub const LOG_MIGRATING_PARTS: &str = "migrating parts for message";
pub const LOG_COPYING: &str = "copying";
pub const LOG_RUNNING_MIGRATION: &str = "running migration";
pub const LOG_MIGRATION_FAILED: &str = "failed to run migration";

/// source: list() — glob "**/*" include file, strip 5-char ".json", split
/// sep, toSorted localeCompare on join("/"). Verbatim rule.
pub fn list_keys(prefix: &[String], files: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = files
        .iter()
        .map(|x| {
            let mut v: Vec<String> = prefix.to_vec();
            let stem = x.strip_suffix(".json").unwrap_or(x);
            v.extend(stem.split('/').map(|s| s.to_string()));
            v
        })
        .collect();
    out.sort_by_key(|a| a.join("/"));
    out
}

/// source: missing() — ENOENT code or reason._tag === "NotFound". Verbatim.
pub fn is_missing_code(code: Option<&str>, reason_tag: Option<&str>) -> bool {
    if code == Some("ENOENT") {
        return true;
    }
    if reason_tag == Some("NotFound") {
        return true;
    }
    false
}

/// source: Interface — remove/read/update/write/list, verbatim.
pub trait Interface {
    fn remove(&self, key: &[String]) -> Result<(), String>;
    fn read_json(&self, key: &[String]) -> Result<serde_json::Value, NotFoundError>;
    fn write_json(&self, key: &[String], content: &serde_json::Value) -> Result<(), String>;
    fn list(&self, prefix: &[String]) -> Result<Vec<Vec<String>>, String>;
}

/// source: Service "@opencode/Storage" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Storage";

/// source: node deps [FSUtil.node, Git.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@opencode-ai/core/fs-util.FSUtil", "@/git.Git"];
