//! Rust port of `packages/app/src/utils/persist.ts` (opencode v1.18.30).
//!
//! Source 704 lines: `Persist` targets (`global`/`workspace`/`draft`/
//! `serverGlobal`/`serverWorkspace`), `removePersisted`, LRU storage cache,
//! quota/fallback handling, legacy migration. SolidJS storage primitives are
//! PROVISIONAL; persist keys and target shapes are verbatim.
//! Original file: `packages/app/src/utils/persist.ts`

#![allow(dead_code)]

/// Mirrors `GLOBAL_STORAGE` / `WINDOW_STORAGE` / `LOCAL_PREFIX` keys (verbatim).
pub const GLOBAL_STORAGE: &str = "opencode.global.dat";
pub const WINDOW_STORAGE: &str = "opencode.window";
pub const LOCAL_PREFIX: &str = "opencode.";
const LEGACY_STORAGE: &str = "default.dat";

/// Mirrors `PersistTarget`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PersistTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "legacyStorageNames", skip_serializing_if = "Option::is_none")]
    pub legacy_storage_names: Option<Vec<String>>,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<Vec<String>>,
}

/// Mirrors `Persist.global(key)`.
pub fn persist_global(key: &str) -> PersistTarget {
    PersistTarget {
        draft: None,
        storage: Some(GLOBAL_STORAGE.to_string()),
        scope: None,
        legacy_storage_names: None,
        key: key.to_string(),
        legacy: None,
    }
}

/// Mirrors `Persist.serverGlobal(scope, key)` — null-byte composition.
pub fn persist_server_global(scope: &str, key: &str) -> PersistTarget {
    if scope == "local" {
        return persist_global(key);
    }
    PersistTarget {
        draft: None,
        storage: Some(GLOBAL_STORAGE.to_string()),
        scope: None,
        legacy_storage_names: None,
        key: format!("{scope}\0{key}"),
        legacy: None,
    }
}

fn checksum(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    let units: Vec<u16> = value.encode_utf16().collect();
    let mut hash: u32 = 0x811c_9dc5;
    for u in units {
        hash ^= u as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    let mut v = hash;
    if v == 0 {
        return Some("0".to_string());
    }
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    while v > 0 {
        out.push(digits[(v % 36) as usize]);
        v /= 36;
    }
    out.reverse();
    Some(String::from_utf8(out).unwrap())
}

fn path_key(dir: &str) -> String {
    dir.replace('\\', "/")
}

/// Mirrors `workspaceStorage(dir)` verbatim: head 12 + checksum.
pub fn workspace_storage(directory: &str) -> String {
    let key = path_key(directory);
    let head_raw = if key.len() >= 12 { &key[..12] } else { &key };
    let head = head_raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();
    let head = if head.is_empty() {
        "workspace".to_string()
    } else {
        head
    };
    let sum = checksum(&key).unwrap_or_else(|| "0".to_string());
    format!("opencode.workspace.{head}.{sum}.dat")
}

fn draft_storage(draft_id: &str) -> String {
    let head_raw = if draft_id.len() >= 12 {
        &draft_id[..12]
    } else {
        draft_id
    };
    let head = head_raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();
    let head = if head.is_empty() {
        "draft".to_string()
    } else {
        head
    };
    let sum = checksum(draft_id).unwrap_or_else(|| "0".to_string());
    format!("opencode.draft.{head}.{sum}.dat")
}

fn window_storage(window_id: &str) -> String {
    let safe = if window_id.is_empty() {
        "browser"
    } else {
        window_id
    };
    let safe = safe
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();
    format!("{WINDOW_STORAGE}.{safe}.dat")
}

/// Mirrors `Persist.workspace(directory, key, legacy?)`.
pub fn persist_workspace(directory: &str, key: &str, legacy: Vec<String>) -> PersistTarget {
    let storage = workspace_storage(&directory.replace('\\', "/"));
    let legacy_storage = workspace_storage(directory);
    let legacy_names = if legacy_storage == storage {
        None
    } else {
        Some(vec![legacy_storage])
    };
    let _ = legacy;
    PersistTarget {
        draft: None,
        storage: Some(storage),
        scope: None,
        legacy_storage_names: legacy_names,
        key: key.to_string(),
        legacy: None,
    }
}

/// Mirrors `Persist.draft(draftId, key)` — per-draft storage isolation (verbatim checksum head).
pub fn persist_draft(draft_id: &str, key: &str) -> PersistTarget {
    PersistTarget {
        draft: Some(true),
        storage: Some(draft_storage(draft_id)),
        scope: None,
        legacy_storage_names: None,
        key: format!("draft:{key}"),
        legacy: None,
    }
}

/// Mirrors `PersistTesting.workspaceStorage` helpers for tests (crate-internal).
pub(crate) fn persist_testing_workspace_storage(dir: &str) -> String {
    workspace_storage(dir)
}
pub(crate) fn persist_testing_window_storage(id: &str) -> String {
    window_storage(id)
}
pub(crate) fn persist_testing_draft_storage(id: &str) -> String {
    draft_storage(id)
}

/// Mirrors `Persist.serverWorkspace(scope, directory, key)`.
pub fn persist_server_workspace(scope: &str, directory: &str, key: &str) -> PersistTarget {
    if scope == "local" {
        return persist_workspace(directory, key, vec![]);
    }
    let mut target = persist_workspace(directory, key, vec![]);
    target.storage = Some(format!(
        "{}:{}\0{}",
        GLOBAL_STORAGE,
        scope,
        target.storage.unwrap_or_default()
    ));
    target.legacy_storage_names = None;
    target.key = format!("{scope}\0{directory}\0{key}");
    target
}

// PROVISIONAL: pending solid-primitives storage backend — mirrors `packages/app/src/utils/persist.ts` reactivity.
#[derive(Debug, Default)]
pub struct PersistedStore {
    pub entries: std::collections::HashMap<String, String>,
}

impl PersistedStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, key: &str, value: &str) {
        self.entries.insert(key.to_string(), value.to_string());
    }

    pub fn transition_remove(&mut self, key: &str) {
        self.entries.remove(key);
    }
}
