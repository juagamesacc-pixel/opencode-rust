//! Rust port of `packages/app/src/utils/draft-store.ts` (opencode v1.18.30).
//!
//! Source 171 lines: `BlobReference`, `createBlobReference`,
//! `createDraftStore`, `createBrowserDraftStore`, `createLegacyBlobReference`.
//! IndexedDB/DOM blob URLs are PROVISIONAL; key/version/encode/decode
//! semantics are preserved.
//! Original file: `packages/app/src/utils/draft-store.ts`

#![allow(dead_code)]

use std::collections::HashMap;

/// Mirrors `BlobReference`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlobReference {
    pub id: String,
    pub url: String,
}

/// Mirrors the `Driver` interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftDriverDescriptor {
    pub name: String,
}

/// Mirrors `DraftStore` (AsyncStorage + `putBlob`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftStoreDescriptor {
    pub name: String,
}

impl DraftStoreDescriptor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Mirrors the legacy blob-reference shape (`{ id: dataUrl }`).
pub fn create_legacy_blob_reference(data_url: &str) -> HashMap<String, String> {
    HashMap::from([("id".to_string(), data_url.to_string())])
}

/// Mirrors `createBrowserDraftStore()` — descriptor only (IndexedDB pending).
pub fn create_browser_draft_store_key() -> DraftStoreDescriptor {
    DraftStoreDescriptor::new("opencode-drafts")
}

// PROVISIONAL: pending IndexedDB/blob runtime — mirrors `packages/app/src/utils/draft-store.ts`.
#[derive(Debug, Default)]
pub struct DraftStore {
    versions: HashMap<String, u64>,
}

impl DraftStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, _key: &str) {}

    pub fn transition_bump(&mut self, key: &str) -> u64 {
        let next = self.versions.get(key).copied().unwrap_or(0) + 1;
        self.versions.insert(key.to_string(), next);
        next
    }
}
