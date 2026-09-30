//! Rust port of `packages/app/src/context/layout-tabs.ts` (opencode v1.18.30).
//!
//! Source 104 lines. Exports: `SESSION_OPEN_FILE_TAB`, `SessionTabs`, `SessionTabState`, `previewSessionTab`, `openSessionTab`, `closeSessionTab`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/layout-tabs.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

pub const SESSION_OPEN_FILE_TAB: &str = "open-file";

/// Mirrors `SessionTabs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionTabs {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `SessionTabState`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionTabState {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `previewSessionTab`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout-tabs.ts
#[allow(non_snake_case)]
pub fn previewSessionTab(/* current: SessionTabState, tab: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `openSessionTab`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout-tabs.ts
#[allow(non_snake_case)]
pub fn openSessionTab(/* current: SessionTabState, tab: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `closeSessionTab`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout-tabs.ts
#[allow(non_snake_case)]
pub fn closeSessionTab(/* current: SessionTabState, tab: string */) -> serde_json::Value {
    serde_json::json!({})
}
