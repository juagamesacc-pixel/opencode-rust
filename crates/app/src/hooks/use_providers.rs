//! Rust port of `packages/app/src/hooks/use-providers.ts` (opencode v1.18.30).
//!
//! Source 75 lines. Exports: `popularProviders`, `useProviders`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/hooks/use-providers.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/hooks/use-providers.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `popularProviders`.
pub const popularProviders_RAW: &str = r#"["#;
// PROVISIONAL: pending solid-js — mirrors packages/app/src/hooks/use-providers.ts
pub fn popularProviders() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `useProviders`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/hooks/use-providers.ts
#[allow(non_snake_case)]
pub fn useProviders(/* directory: Accessor<string | undefined> */) -> serde_json::Value {
    serde_json::json!({})
}
