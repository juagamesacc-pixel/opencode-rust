//! Rust port of `packages/app/src/context/language.tsx` (opencode v1.18.30).
//!
//! Source 243 lines. Exports: `Locale`, `Direction`, `loadLocaleDict`, `normalizeLocale`, `loadInitialLocale`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/language.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/language.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `Locale`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locale {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Direction`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Direction {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `loadLocaleDict`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/language.tsx
#[allow(non_snake_case)]
pub fn loadLocaleDict(/* locale: Locale */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `normalizeLocale`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/language.tsx
#[allow(non_snake_case)]
pub fn normalizeLocale(/* value: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `loadInitialLocale`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/language.tsx
#[allow(non_snake_case)]
pub fn loadInitialLocale() -> serde_json::Value {
    serde_json::json!({})
}
