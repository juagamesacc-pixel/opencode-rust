// source: packages/enterprise/src/core/storage.ts — exports: Storage namespace (Adapter, read/write/remove/list/update)
//! 1:1 port — key resolution, prefix mapping, XML key extraction, adapter selection, and
//! before/after filtering preserved verbatim. HTTP transport is PROVISIONAL (aws4fetch SigV4
//! has no equivalent here) — modeled as a `Store` trait with an in-memory implementation.

use std::collections::HashMap;

/// source: default region `"us-east-1"` verbatim
pub const DEFAULT_REGION: &str = "us-east-1";
/// source: `Content-Type: application/json` verbatim
pub const CONTENT_TYPE_JSON: &str = "application/json";
/// source: `"No storage adapter configured"` verbatim
pub const NO_ADAPTER_MSG: &str = "No storage adapter configured";
/// source: `update` missing-value error `"Not found"` verbatim
pub const NOT_FOUND_MSG: &str = "Not found";
/// source: env var names verbatim
pub const ENV_BUCKET: &str = "OPENCODE_STORAGE_BUCKET";
pub const ENV_REGION: &str = "OPENCODE_STORAGE_REGION";
pub const ENV_ACCESS_KEY_ID: &str = "OPENCODE_STORAGE_ACCESS_KEY_ID";
pub const ENV_SECRET_ACCESS_KEY: &str = "OPENCODE_STORAGE_SECRET_ACCESS_KEY";
pub const ENV_ACCOUNT_ID: &str = "OPENCODE_STORAGE_ACCOUNT_ID";
pub const ENV_ADAPTER: &str = "OPENCODE_STORAGE_ADAPTER";

/// source: `Storage.Adapter` — read/write/remove/list surface verbatim.
/// Sync here (source is async over HTTP); ordering and semantics preserved.
pub trait Store {
    fn read(&self, path: &str) -> Result<Option<String>, String>;
    fn write(&mut self, path: &str, value: String) -> Result<(), String>;
    fn remove(&mut self, path: &str) -> Result<(), String>;
    fn list(
        &self,
        prefix: &str,
        limit: Option<usize>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<Vec<String>, String>;
}

/// source: `resolve` — `key.join("/") + ".json"` verbatim
pub fn resolve(key: &[&str]) -> String {
    format!("{}.json", key.join("/"))
}

/// source: `list` prefix mapping — `prefix.join("/") + (prefix.length ? "/" : "")` verbatim
pub fn list_prefix(prefix_parts: &[&str]) -> String {
    if prefix_parts.is_empty() {
        String::new()
    } else {
        format!("{}/", prefix_parts.join("/"))
    }
}

/// source: `list` result mapping — strip `.json`, split `/` verbatim
pub fn split_key(key: &str) -> Vec<String> {
    key.strip_suffix(".json")
        .unwrap_or(key)
        .split('/')
        .map(str::to_string)
        .collect()
}

/// source: `createAdapter().list` XML parsing — `/<Key>([^<]+)<\/Key>/g` verbatim, no regex dep
pub fn extract_keys(xml: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut rest = xml;
    while let Some(start) = rest.find("<Key>") {
        let after = &rest[start + 5..];
        if let Some(end) = after.find("</Key>") {
            keys.push(after[..end].to_string());
            rest = &after[end + 6..];
        } else {
            break;
        }
    }
    keys
}

/// source: `createAdapter().list` after/before semantics verbatim —
/// `start-after = prefix + after + ".json"`, keep `key < prefix + before + ".json"`
pub fn apply_bounds(
    keys: Vec<String>,
    prefix: &str,
    after: Option<&str>,
    before: Option<&str>,
) -> Vec<String> {
    keys.into_iter()
        .filter(|key| {
            if let Some(after) = after {
                let bound = format!("{prefix}{after}.json");
                if key.as_str() <= bound.as_str() {
                    return false;
                }
            }
            if let Some(before) = before {
                let bound = format!("{prefix}{before}.json");
                if key.as_str() >= bound.as_str() {
                    return false;
                }
            }
            true
        })
        .collect()
}

/// source: `s3()` endpoint `https://s3.${region}.amazonaws.com` verbatim
pub fn s3_base(region: &str) -> String {
    format!("https://s3.{region}.amazonaws.com")
}

/// source: `r2()` endpoint `https://${accountId}.r2.cloudflarestorage.com` verbatim
pub fn r2_base(account_id: &str) -> String {
    format!("https://{account_id}.r2.cloudflarestorage.com")
}

/// source: storage adapter selection — `"r2"` / `"s3"` / throw verbatim
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterKind {
    R2,
    S3,
}

pub fn select_adapter(kind: Option<&str>) -> Result<AdapterKind, String> {
    match kind {
        Some("r2") => Ok(AdapterKind::R2),
        Some("s3") => Ok(AdapterKind::S3),
        _ => Err(NO_ADAPTER_MSG.to_string()),
    }
}

/// In-memory `Store` — same key/filter semantics as the S3/R2 adapter, for tests and local use.
#[derive(Debug, Default)]
pub struct MemoryStore {
    map: HashMap<String, String>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl Store for MemoryStore {
    fn read(&self, path: &str) -> Result<Option<String>, String> {
        Ok(self.map.get(path).cloned())
    }

    fn write(&mut self, path: &str, value: String) -> Result<(), String> {
        self.map.insert(path.to_string(), value);
        Ok(())
    }

    fn remove(&mut self, path: &str) -> Result<(), String> {
        self.map.remove(path);
        Ok(())
    }

    fn list(
        &self,
        prefix: &str,
        limit: Option<usize>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<Vec<String>, String> {
        let mut keys: Vec<String> = self
            .map
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        keys.sort();
        keys = apply_bounds(keys, prefix, after, before);
        if let Some(limit) = limit {
            keys.truncate(limit);
        }
        Ok(keys)
    }
}
