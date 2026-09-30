//! Rust port of `src/main/store-cleanup.ts` (opencode v1.18.30).
//!
//! The retention / recency / emptiness **decision logic** is reproduced
//! 1:1 over an injected entry snapshot, so it is unit-testable without `fs`:
//! [`store_kind`], [`is_empty_store`], [`plan_cleanup`] and
//! [`should_delete_if_empty`] carry the exact predicates and ordering of the
//! source. The two entry points — [`cleanup_store_files] and
//! [`delete_store_file_if_empty`] — use `std::fs` to mirror the source's
//! `readdir`/`stat`/`readFile`/`rm` chain (`node:fs/promises` has no binding).
//!
//! Original file: `packages/desktop/src/main/store-cleanup.ts`

use std::collections::BTreeSet;

use crate::main::store_keys::SETTINGS_STORE;

/// Mirrors `const EMPTY_STORE_MAX_BYTES = 128`.
pub const EMPTY_STORE_MAX_BYTES: u64 = 128;

/// Mirrors `const DRAFT_RETENTION_MS = 30 * 24 * 60 * 60 * 1000`.
pub const DRAFT_RETENTION_MS: f64 = 30.0 * 24.0 * 60.0 * 60.0 * 1000.0;

/// Mirrors `const DRAFT_KEEP_RECENT = 100`.
pub const DRAFT_KEEP_RECENT: usize = 100;

/// Mirrors `type StoreKind = "draft" | "workspace"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoreKind {
    Draft,
    Workspace,
}

impl StoreKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            StoreKind::Draft => "draft",
            StoreKind::Workspace => "workspace",
        }
    }
}

/// Mirrors `type StoreCandidate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreCandidate {
    pub name: String,
    pub path: String,
    pub kind: StoreKind,
    pub modified: f64,
    pub empty: bool,
}

/// Mirrors `function storeKind(name)`: `/^opencode\.draft\..+\.dat$/` then
/// `/^opencode\.workspace\..+\.dat$/`. The `.+` requires at least one
/// character between the prefix and `.dat`.
pub fn store_kind(name: &str) -> Option<StoreKind> {
    if scoped_matches(name, "opencode.draft.") {
        return Some(StoreKind::Draft);
    }
    if scoped_matches(name, "opencode.workspace.") {
        return Some(StoreKind::Workspace);
    }
    None
}

fn scoped_matches(name: &str, prefix: &str) -> bool {
    let Some(rest) = name.strip_prefix(prefix) else {
        return false;
    };
    match rest.strip_suffix(".dat") {
        Some(middle) => !middle.is_empty(),
        None => false,
    }
}

/// Mirrors `function isEmptyStore(file, size)`.
///
/// `size > 128` short-circuits to `false` without reading the file. A body
/// that trims to the empty string is empty; otherwise the JSON must parse to
/// a non-array object with no own keys.
pub fn is_empty_store(raw: Option<&str>, size: u64) -> bool {
    if size > EMPTY_STORE_MAX_BYTES {
        return false;
    }
    let Some(raw) = raw else {
        return false;
    };
    if raw.trim().is_empty() {
        return true;
    }
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(raw) else {
        return false;
    };
    match parsed {
        serde_json::Value::Object(record) => record.is_empty(),
        _ => false,
    }
}

/// Mirrors the `stale` `Set` accumulation plus the recency cap in
/// `cleanupStoreFiles`.
///
/// The `Set` iteration order is insertion order in the source, which is the
/// order the candidates appear in `readdir` — reproduced by [`plan_cleanup`]
/// returning candidates in input order.
///
/// The source sorts by `b.modified - a.modified` (descending) and then
/// `slice(100)`, marking everything past the cap stale. `sort` is stable in
/// JS, so equal mtimes keep their readdir order.
pub fn plan_cleanup(candidates: &[StoreCandidate], now: f64) -> Vec<StoreCandidate> {
    let mut stale: Vec<StoreCandidate> = Vec::new();
    let mut push = |candidate: &StoreCandidate, stale: &mut Vec<StoreCandidate>| {
        if !stale.iter().any(|existing| existing.name == candidate.name) {
            stale.push(candidate.clone());
        }
    };
    for candidate in candidates {
        if candidate.empty {
            push(candidate, &mut stale);
        }
        if candidate.kind == StoreKind::Draft && now - candidate.modified > DRAFT_RETENTION_MS {
            push(candidate, &mut stale);
        }
    }

    let mut drafts: Vec<&StoreCandidate> = candidates
        .iter()
        .filter(|candidate| candidate.kind == StoreKind::Draft && !candidate.empty)
        .collect();
    drafts.sort_by(|a, b| {
        b.modified
            .partial_cmp(&a.modified)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for candidate in drafts.iter().skip(DRAFT_KEEP_RECENT) {
        push(candidate, &mut stale);
    }

    stale
}

/// Mirrors the `{ scanned, deleted }` result of `cleanupStoreFiles`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupResult {
    pub scanned: usize,
    pub deleted: Vec<String>,
}

/// Mirrors the delete half of `cleanupStoreFiles`: the plan is ported above,
/// the `rm(candidate.path, { force: true })` is PROVISIONAL.
pub fn deleted_names(candidates: &[StoreCandidate], now: f64) -> CleanupResult {
    let plan = plan_cleanup(candidates, now);
    CleanupResult {
        scanned: candidates.len(),
        deleted: plan.into_iter().map(|candidate| candidate.name).collect(),
    }
}

/// Mirrors `deleteStoreFileIfEmpty(userDataPath, name)`, minus the `fs` calls.
pub fn should_delete_if_empty(name: &str, raw: Option<&str>, size: Option<u64>) -> bool {
    if store_kind(name).is_none() {
        return false;
    }
    let Some(size) = size else {
        return false;
    };
    is_empty_store(raw, size)
}

/// Mirrors `deleteStoreFileIfEmpty(userDataPath, name)`.
///
/// The source's `stat(file)` (`catch → undefined`), `isFile()`, capped
/// `isEmptyStore` read, and `rm(file, { force: true })` chain is ported onto
/// `std::fs`; the result maps to `Ok(())` for both the delete and the no-op
/// paths (the store cache drops the name either way), `Err` only on a failed
/// removal.
pub fn delete_store_file_if_empty(user_data_path: &str, name: &str) -> Result<(), String> {
    if store_kind(name).is_none() {
        return Ok(());
    }
    let path = std::path::Path::new(user_data_path).join(name);
    let Ok(metadata) = std::fs::metadata(&path) else {
        // `stat(file).catch(() => undefined)` → `!stats?.isFile()` short-circuits.
        return Ok(());
    };
    if !metadata.is_file() {
        return Ok(());
    }
    let size = metadata.len();
    if size > EMPTY_STORE_MAX_BYTES {
        return Ok(());
    }
    let raw = std::fs::read_to_string(&path).ok();
    if !is_empty_store(raw.as_deref(), size) {
        return Ok(());
    }
    std::fs::remove_file(&path)
        .map_err(|error| format!("failed to remove {}: {error}", path.to_string_lossy()))
}

/// Mirrors `cleanupStoreFiles(userDataPath)` — the `now = Date.now()`
/// default of the source, in epoch milliseconds.
pub fn cleanup_store_files(user_data_path: &str) -> CleanupResult {
    cleanup_store_files_at(user_data_path, now_ms())
}

/// [`cleanup_store_files`] with an injected `now` so the plan is testable
/// deterministically (the source signature is `(userDataPath, now = Date.now())`).
pub fn cleanup_store_files_at(user_data_path: &str, now: f64) -> CleanupResult {
    let entries = read_dir_entries(user_data_path);
    let candidates: Vec<StoreCandidate> = entries
        .iter()
        .filter(|entry| entry.is_file)
        .filter_map(|entry| store_kind(&entry.name).map(|kind| (entry, kind)))
        .map(|(entry, kind)| StoreCandidate {
            name: entry.name.clone(),
            path: entry.path.clone(),
            kind,
            modified: entry.modified,
            empty: read_empty_store_file(entry),
        })
        .collect();

    let plan = plan_cleanup(&candidates, now);
    let mut deleted = Vec::new();
    for candidate in &plan {
        // `rm(candidate.path, { force: true })` — force ignores missing files.
        if std::fs::remove_file(&candidate.path).is_ok() {
            deleted.push(candidate.name.clone());
        }
    }
    CleanupResult {
        scanned: candidates.len(),
        deleted,
    }
}

/// `Date.now()` in epoch milliseconds.
pub fn now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as f64)
        .unwrap_or(0.0)
}

fn mtime_ms(metadata: &std::fs::Metadata) -> f64 {
    metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis() as f64)
        .unwrap_or(0.0)
}

/// One `readdirSync(userDataPath, { withFileTypes: true })` entry with the
/// fields the source touches: `isFile()`, `name`, `stat().mtimeMs`, `size`.
struct DirEntry {
    name: String,
    path: String,
    is_file: bool,
    size: u64,
    modified: f64,
}

fn read_dir_entries(user_data_path: &str) -> Vec<DirEntry> {
    let Ok(read_dir) = std::fs::read_dir(user_data_path) else {
        // `.catch(() => [])`
        return Vec::new();
    };
    read_dir
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            Some(DirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: entry.path().to_string_lossy().into_owned(),
                is_file: metadata.is_file(),
                size: metadata.len(),
                modified: mtime_ms(&metadata),
            })
        })
        .collect()
}

/// Mirrors `isEmptyStore(file, stats.size)`: the file is only read under the
/// 128-byte cap, exactly as the source does.
fn read_empty_store_file(entry: &DirEntry) -> bool {
    if entry.size > EMPTY_STORE_MAX_BYTES {
        return false;
    }
    let raw = std::fs::read_to_string(&entry.path).ok();
    is_empty_store(raw.as_deref(), entry.size)
}

/// The default store name, re-exported so callers do not need two imports.
pub fn default_store_name() -> &'static str {
    SETTINGS_STORE
}

/// Mirrors the `new Set<StoreCandidate>()` name-collector used by the plan.
pub fn stale_names(candidates: &[StoreCandidate], now: f64) -> BTreeSet<String> {
    plan_cleanup(candidates, now)
        .into_iter()
        .map(|candidate| candidate.name)
        .collect()
}

#[cfg(test)]
mod tests {
    // Mirrors `src/main/store-cleanup.test.ts` (`describe("store cleanup")`).
    // The source exercises the exported functions against a real temp
    // directory; the decision tables below drive the identical functions
    // over an injected candidate snapshot, plus a real-`std::fs` round trip
    // for the entry points.
    use super::*;

    /// `2026-07-01T00:00:00.000Z` in epoch milliseconds, as the source's
    /// `now`.
    const NOW: f64 = 1_783_670_400_000.0;

    fn candidate(name: &str, modified: f64, empty: bool) -> StoreCandidate {
        StoreCandidate {
            name: name.to_string(),
            path: format!("/tmp/root/{}", name),
            kind: store_kind(name).expect("scoped store"),
            modified,
            empty,
        }
    }

    #[test]
    fn removes_empty_scoped_stores_and_leaves_global_stores_alone() {
        // The source writes opencode.draft.empty.dat, opencode.workspace.empty.dat,
        // opencode.global.dat and opencode.workspace.empty.dat.json.
        let candidates = vec![
            candidate("opencode.draft.empty.dat", NOW, true),
            candidate("opencode.workspace.empty.dat", NOW, true),
        ];
        // `opencode.global.dat` and `opencode.workspace.empty.dat.json` are
        // never candidates: `storeKind` returns undefined for both.
        assert_eq!(store_kind("opencode.global.dat"), None);
        assert_eq!(store_kind("opencode.workspace.empty.dat.json"), None);
        assert_eq!(
            store_kind("opencode.draft..dat"),
            None,
            "`.+` needs a character"
        );

        let mut deleted = deleted_names(&candidates, NOW).deleted;
        deleted.sort();
        assert_eq!(
            deleted,
            vec!["opencode.draft.empty.dat", "opencode.workspace.empty.dat"]
        );
    }

    #[test]
    fn removes_stale_drafts_by_age_without_removing_non_empty_workspace_stores() {
        // 2026-05-01 and 2025-01-01, per the source's `writeStore` calls.
        let candidates = vec![
            candidate("opencode.draft.old.dat", 1_777_593_600_000.0, false),
            candidate("opencode.draft.recent.dat", NOW, false),
            candidate("opencode.workspace.old.dat", 1_735_708_800_000.0, false),
            candidate("opencode.workspace.recent.dat", NOW, false),
        ];

        assert_eq!(
            deleted_names(&candidates, NOW).deleted,
            vec!["opencode.draft.old.dat"]
        );
    }

    #[test]
    fn caps_scoped_stores_by_recency() {
        let candidates: Vec<StoreCandidate> = (0..102)
            .map(|index| {
                candidate(
                    &format!("opencode.draft.{}.dat", index),
                    NOW - index as f64 * 1000.0,
                    false,
                )
            })
            .collect();

        let result = deleted_names(&candidates, NOW);
        let mut deleted = result.deleted.clone();
        deleted.sort();
        assert_eq!(
            deleted,
            vec!["opencode.draft.100.dat", "opencode.draft.101.dat"]
        );
        assert_eq!(result.scanned - result.deleted.len(), DRAFT_KEEP_RECENT);
    }

    #[test]
    fn removes_a_scoped_store_immediately_when_it_becomes_empty() {
        assert!(should_delete_if_empty(
            "opencode.draft.empty.dat",
            Some("{}"),
            Some(2)
        ));
        assert!(!should_delete_if_empty(
            "opencode.global.dat",
            Some("{}"),
            Some(2)
        ));
        assert!(
            !should_delete_if_empty("opencode.draft.x.dat", Some("{}"), None),
            "a missing stat returns false"
        );
    }

    #[test]
    fn emptiness_follows_size_trim_and_json_shape() {
        assert!(is_empty_store(Some(""), 0));
        assert!(is_empty_store(Some("  \n "), 4));
        assert!(is_empty_store(Some("{}"), 2));
        assert!(is_empty_store(Some("{\n}"), 4));
        assert!(
            !is_empty_store(Some("[]"), 2),
            "arrays are not empty stores"
        );
        assert!(!is_empty_store(Some("{\"a\":1}"), 7));
        assert!(!is_empty_store(Some("not json"), 8));
        assert!(!is_empty_store(None, 2), "an unreadable file is not empty");
        assert!(
            !is_empty_store(Some("{}"), 129),
            "over the 128-byte cap it is not even read"
        );
    }

    #[test]
    fn an_empty_draft_counts_as_stale_only_once() {
        let candidates = vec![candidate(
            "opencode.draft.empty.dat",
            NOW - 40.0 * 24.0 * 3600.0 * 1000.0,
            true,
        )];
        assert_eq!(
            deleted_names(&candidates, NOW).deleted,
            vec!["opencode.draft.empty.dat"]
        );
    }

    fn temp_dir(tag: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("oc-store-cleanup-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn write_store(dir: &std::path::Path, name: &str, contents: &str) {
        std::fs::write(dir.join(name), contents).expect("write store file");
    }

    #[test]
    fn cleanup_removes_empty_and_stale_scoped_files_in_a_real_dir() {
        let dir = temp_dir("cleanup");
        write_store(&dir, "opencode.draft.empty.dat", "{}");
        write_store(&dir, "opencode.draft.recent.dat", "{\"a\":1}");
        write_store(&dir, "opencode.workspace.old.dat", "{\"b\":2}");
        write_store(&dir, "opencode.draft.big.dat", &"x".repeat(200));
        write_store(&dir, "random.txt", "not a scoped store");

        // A far-future `now` makes even fresh non-empty drafts stale by age.
        let result = cleanup_store_files_at(dir.to_str().unwrap(), 1e15);

        assert!(result
            .deleted
            .contains(&"opencode.draft.empty.dat".to_string()));
        assert!(result
            .deleted
            .contains(&"opencode.draft.recent.dat".to_string()));
        assert!(!dir.join("opencode.draft.empty.dat").exists());
        assert!(!dir.join("opencode.draft.recent.dat").exists());
        // Non-empty workspace stores are never age-deleted; the oversized
        // draft is not empty; the unscoped file is not a candidate.
        assert!(dir.join("opencode.workspace.old.dat").exists());
        assert!(dir.join("opencode.draft.big.dat").exists());
        assert!(dir.join("random.txt").exists());
        assert_eq!(result.scanned, 4, "empty + recent + workspace + big");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn delete_store_file_if_empty_removes_only_empty_scoped_files() {
        let dir = temp_dir("delete-if-empty");
        write_store(&dir, "opencode.draft.empty.dat", "");
        write_store(&dir, "opencode.draft.full.dat", "{\"a\":1}");
        write_store(&dir, "opencode.global.dat", "{}");

        let path = |name: &str| dir.join(name).to_string_lossy().into_owned();
        assert!(delete_store_file_if_empty(
            &path("opencode.draft.empty.dat"),
            "opencode.draft.empty.dat"
        )
        .is_ok());
        assert!(!dir.join("opencode.draft.empty.dat").exists());
        assert!(delete_store_file_if_empty(
            &path("opencode.draft.full.dat"),
            "opencode.draft.full.dat"
        )
        .is_ok());
        assert!(
            dir.join("opencode.draft.full.dat").exists(),
            "non-empty stays"
        );
        assert!(
            delete_store_file_if_empty(&path("opencode.global.dat"), "opencode.global.dat").is_ok()
        );
        assert!(dir.join("opencode.global.dat").exists(), "unscoped stays");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
