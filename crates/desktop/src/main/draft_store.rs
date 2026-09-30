//! Rust port of `src/main/draft-store.ts` (opencode v1.18.30).
//!
//! The SQLite execution (`node:sqlite` `DatabaseSync` + `drizzle-orm`) has
//! no in-workspace binding and stays PROVISIONAL
//! (`create_desktop_draft_store`, `DesktopDraftStore`). Fully ported:
//! - `SCHEMA_SQL`: the `native.exec(…)` statement, byte-identical.
//! - `FLUSH_DEBOUNCE_MS`: the 500 ms `setTimeout(flush, 500)` delay.
//! - `PendingWrites`: the `pending` map plus the flush snapshot ordering
//!   (`[...pending]` then `clear()` before applying).
//! - `collect_referenced_blob_ids`: the startup GC walk that gathers
//!   `item.blob.id` strings from every stored document value (mirrors the
//!   `JSON.parse` reviver).
//! - `putBlob` ids are sha256 hex (`node:crypto` `createHash`); computing
//!   them needs `sha2`, which is not in the workspace, so id derivation is
//!   PROVISIONAL.
//!
//! Original file: `packages/desktop/src/main/draft-store.ts`

use std::collections::{HashMap, HashSet};

pub const SCHEMA_SQL: &str = "PRAGMA journal_mode=WAL; CREATE TABLE IF NOT EXISTS document (key TEXT PRIMARY KEY, value TEXT NOT NULL); CREATE TABLE IF NOT EXISTS blob (id TEXT PRIMARY KEY, data BLOB NOT NULL);";

pub const FLUSH_DEBOUNCE_MS: u64 = 500;

/// Mirrors the `pending: Map<string, string | null>` buffer. `None` is a
/// tombstone (mirrors `set(key, null)` → `tx.delete(…)` on flush).
#[derive(Debug, Default)]
pub struct PendingWrites {
    pending: HashMap<String, Option<String>>,
}

impl PendingWrites {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<Option<&str>> {
        self.pending.get(key).map(|value| value.as_deref())
    }

    pub fn set(&mut self, key: String, value: Option<String>) {
        self.pending.insert(key, value);
    }

    /// Mirrors `const writes = [...pending]; pending.clear()`.
    pub fn take_writes(&mut self) -> Vec<(String, Option<String>)> {
        let writes: Vec<(String, Option<String>)> = self
            .pending
            .drain()
            .map(|(key, value)| (key, value))
            .collect();
        writes
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

/// Mirrors the startup walk that collects referenced `item.blob.id`
/// strings from every `document` value before deleting orphan blobs.
pub fn collect_referenced_blob_ids(values: &[&str]) -> HashSet<String> {
    let mut used = HashSet::new();
    for value in values {
        let parsed: serde_json::Value = match serde_json::from_str(value) {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };
        collect_blob_ids(&parsed, &mut used);
    }
    used
}

fn collect_blob_ids(value: &serde_json::Value, used: &mut HashSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            // Mirrors the reviver check `item?.blob &&
            // typeof item.blob.id === "string"`, applied to every nested
            // value exactly as `JSON.parse` visits them.
            if let Some(blob) = map.get("blob") {
                if let Some(id) = blob.get("id").and_then(|id| id.as_str()) {
                    used.insert(id.to_string());
                }
            }
            for item in map.values() {
                collect_blob_ids(item, used);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_blob_ids(item, used);
            }
        }
        _ => {}
    }
}

// PROVISIONAL(packages/desktop/src/main/draft-store.ts): needs
// `node:sqlite` (`DatabaseSync`) + `drizzle-orm`. Signature mirrors
// `createDesktopDraftStore(filename)`.
pub struct DesktopDraftStore {
    _private: (),
}

impl DesktopDraftStore {
    pub fn get(&self, _key: &str) -> Option<String> {
        unimplemented!("node:sqlite binding")
    }

    pub fn set(&mut self, _key: String, _value: Option<String>) {
        unimplemented!("node:sqlite binding")
    }

    pub fn put_blob(&mut self, _data: &[u8]) -> String {
        unimplemented!("node:sqlite binding")
    }

    pub fn get_blob(&self, _id: &str) -> Option<Vec<u8>> {
        unimplemented!("node:sqlite binding")
    }

    pub fn flush(&mut self) {
        unimplemented!("node:sqlite binding")
    }

    pub fn close(mut self) {
        self.flush();
    }
}

// PROVISIONAL(packages/desktop/src/main/draft-store.ts): needs
// `node:sqlite` (`DatabaseSync`) + `drizzle-orm`.
pub fn create_desktop_draft_store(_filename: &str) -> DesktopDraftStore {
    unimplemented!("node:sqlite binding")
}

#[cfg(test)]
mod tests {
    // `src/main/draft-store.test.ts` drives a real `:memory:` SQLite
    // database, which has no executable equivalent while the store is
    // PROVISIONAL (recorded as skipped-with-reason). The cases below cover
    // the ported halves with the same intent: latest buffered write wins,
    // and blob GC keeps referenced ids while dropping orphans.
    use super::*;

    #[test]
    fn flushes_the_latest_buffered_draft() {
        let mut pending = PendingWrites::new();
        pending.set("prompt".to_string(), Some("first".to_string()));
        pending.set("prompt".to_string(), Some("latest".to_string()));
        assert_eq!(pending.get("prompt"), Some(Some("latest")));
        let writes = pending.take_writes();
        assert_eq!(
            writes,
            vec![("prompt".to_string(), Some("latest".to_string()))]
        );
        assert!(pending.is_empty());
    }

    #[test]
    fn collects_referenced_blob_ids_for_gc() {
        let values = [
            r#"{"text":"hi","image":{"blob":{"id":"abc"}}}"#,
            r#"{"nested":[{"blob":{"id":42}}]}"#,
            "not json",
        ];
        let used = collect_referenced_blob_ids(&values);
        assert_eq!(used, HashSet::from(["abc".to_string()]));
    }
}
