//! Rust port of `packages/app/src/context/layout.tsx` (opencode v1.18.30).
//!
//! Source 1082 lines. Exports: `AvatarColorKey`, `getAvatarColors`, `getProjectAvatarVariant`, `LocalProject`, `HomeProjectSelection`, `ReviewDiffStyle`, `ReviewChangeMode`, `ReviewPanelSource`, `LayoutRoute`, `currentRoute`, `createSessionKeyReader`, `ensureSessionKey`, `pruneSessionKeys`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/layout.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/layout.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `AvatarColorKey`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvatarColorKey {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `getAvatarColors`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout.tsx
#[allow(non_snake_case)]
pub fn getAvatarColors(/* key?: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `getProjectAvatarVariant`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout.tsx
#[allow(non_snake_case)]
pub fn getProjectAvatarVariant(/* key?: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `LocalProject`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalProject {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `HomeProjectSelection`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeProjectSelection {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `ReviewDiffStyle`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewDiffStyle {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `ReviewChangeMode`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewChangeMode {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `ReviewPanelSource`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewPanelSource {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `LayoutRoute`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutRoute {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `currentRoute`.
pub fn currentRoute_value() -> String {
    String::new()
}

/// Mirrors `createSessionKeyReader`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout.tsx
#[allow(non_snake_case)]
pub fn createSessionKeyReader() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `ensureSessionKey`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout.tsx
#[allow(non_snake_case)]
pub fn ensureSessionKey() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `pruneSessionKeys`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/layout.tsx
#[allow(non_snake_case)]
pub fn pruneSessionKeys() -> serde_json::Value {
    serde_json::json!({})
}
