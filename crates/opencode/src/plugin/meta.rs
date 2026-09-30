// source: src/plugin/meta.ts — exports: Theme, Entry, State, Touch,
// touchMany, touch, setTheme, list, PluginMeta
// PROVISIONAL pending core (flag, global, util/flock) + @/util/filesystem:
// store filename, lock prefix, fingerprint joins, next() state rules,
// "Failed to touch plugin metadata." verbatim.

use serde::{Deserialize, Serialize};

/// source: Theme { src, dest, mtime?, size? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub src: String,
    pub dest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

/// source: Entry — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub source: String,
    pub spec: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<i64>,
    pub first_time: i64,
    pub last_time: i64,
    pub time_changed: i64,
    pub load_count: u64,
    pub fingerprint: String,
}

/// source: State literals — verbatim.
pub const STATE_FIRST: &str = "first";
pub const STATE_UPDATED: &str = "updated";
pub const STATE_SAME: &str = "same";

/// source: store file "plugin-meta.json" — verbatim.
pub const META_FILE: &str = "plugin-meta.json";
/// source: OPENCODE_PLUGIN_META_FILE flag — verbatim key.
pub const META_FILE_ENV: &str = "OPENCODE_PLUGIN_META_FILE";

/// source: lock `plugin-meta:${file}` — verbatim.
pub fn lock_key(file: &str) -> String {
    format!("plugin-meta:{}", file)
}

/// source: fingerprint() — file: target|modified; npm: target|requested|version. Verbatim.
pub fn fingerprint_file(target: &str, modified: Option<i64>) -> String {
    format!(
        "{}|{}",
        target,
        modified.map(|m| m.to_string()).unwrap_or_default()
    )
}

/// source: fingerprint() npm branch — verbatim.
pub fn fingerprint_npm(target: &str, requested: Option<&str>, version: Option<&str>) -> String {
    format!(
        "{}|{}|{}",
        target,
        requested.unwrap_or(""),
        version.unwrap_or("")
    )
}

/// source: next() — first_time ?? now; load_count+1; same/updated by
/// fingerprint; updated bumps time_changed. Verbatim.
pub fn next_state(prev_fingerprint: Option<&str>, fingerprint: &str) -> &'static str {
    match prev_fingerprint {
        None => STATE_FIRST,
        Some(p) if p == fingerprint => STATE_SAME,
        Some(_) => STATE_UPDATED,
    }
}

/// source: "Failed to touch plugin metadata." — verbatim.
pub const TOUCH_FAILED_MESSAGE: &str = "Failed to touch plugin metadata.";
