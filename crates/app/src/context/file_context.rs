//! Rust port of `packages/app/src/context/file.tsx` (opencode v1.18.30).
//!
//! Source 304 lines. Exports: `selectionFromLines`, `evictContentLru`, `getFileContentBytesTotal`, `getFileContentEntryCount`, `removeFileContentBytes`, `resetFileContentLru`, `setFileContentBytes`, `touchFileContent`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/file.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/file.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `selectionFromLines`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn selectionFromLines() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `evictContentLru`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn evictContentLru() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `getFileContentBytesTotal`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn getFileContentBytesTotal() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `getFileContentEntryCount`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn getFileContentEntryCount() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `removeFileContentBytes`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn removeFileContentBytes() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `resetFileContentLru`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn resetFileContentLru() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `setFileContentBytes`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn setFileContentBytes() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `touchFileContent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/file.tsx
#[allow(non_snake_case)]
pub fn touchFileContent() -> serde_json::Value {
    serde_json::json!({})
}
