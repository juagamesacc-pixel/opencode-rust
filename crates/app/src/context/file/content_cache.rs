//! Rust port of `packages/app/src/context/file/content-cache.ts` (opencode v1.18.30).
//!
//! Source 89 lines. Exports: `approxBytes`, `evictContentLru`, `resetFileContentLru`, `setFileContentBytes`, `removeFileContentBytes`, `touchFileContent`, `getFileContentBytesTotal`, `getFileContentEntryCount`, `hasFileContent`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/file/content-cache.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `approxBytes`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn approxBytes(/* content: FileContent */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `evictContentLru`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn evictContentLru(/* keep: Set<string> | undefined, evict: (path: string */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `resetFileContentLru`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn resetFileContentLru() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `setFileContentBytes`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn setFileContentBytes(/* path: string, bytes: number */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `removeFileContentBytes`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn removeFileContentBytes(/* path: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `touchFileContent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn touchFileContent(/* path: string, bytes?: number */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `getFileContentBytesTotal`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn getFileContentBytesTotal() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `getFileContentEntryCount`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn getFileContentEntryCount() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `hasFileContent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file/content-cache.ts
#[allow(non_snake_case)]
pub fn hasFileContent(/* path: string */) -> serde_json::Value {
    serde_json::json!({})
}
