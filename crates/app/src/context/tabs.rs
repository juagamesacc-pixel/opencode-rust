//! Rust port of `packages/app/src/context/tabs.tsx` (opencode v1.18.30).
//!
//! Source 386 lines. Exports: `SessionTab`, `DraftTab`, `Tab`, `TabInfo`, `draftHref`, `tabHref`, `tabKey`, `sessionHasOpenTab`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/tabs.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/tabs.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `SessionTab`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionTab {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `DraftTab`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DraftTab {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Tab`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tab {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `TabInfo`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabInfo {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `draftHref`.
pub fn draftHref_value() -> String {
    String::new()
}

/// Mirrors `tabHref`.
pub fn tabHref_value() -> String {
    String::new()
}

/// Mirrors `tabKey`.
pub fn tabKey_value() -> String {
    String::new()
}

/// Mirrors `sessionHasOpenTab`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/tabs.tsx
#[allow(non_snake_case)]
pub fn sessionHasOpenTab(/* tabs: Tab[], server: ServerConnection.Key, session: Session */
) -> serde_json::Value {
    serde_json::json!({})
}
