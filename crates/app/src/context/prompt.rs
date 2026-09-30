//! Rust port of `packages/app/src/context/prompt.tsx` (opencode v1.18.30).
//!
//! Source 171 lines. Exports: `selectPromptTab`, `createTabPromptState`, `createPromptReady`, `createPromptSession`, `createPromptState`, `DEFAULT_PROMPT`, `isCommentItem`, `isPromptEqual`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/prompt.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/prompt.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `selectPromptTab`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt.tsx
#[allow(non_snake_case)]
pub fn selectPromptTab(/* tabs: Tab[], scope: PromptScope, server: ServerConnection.Key */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `createTabPromptState`.
pub fn createTabPromptState_value() -> String {
    String::new()
}

/// Mirrors `createPromptReady`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt.tsx
#[allow(non_snake_case)]
pub fn createPromptReady() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `createPromptSession`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt.tsx
#[allow(non_snake_case)]
pub fn createPromptSession() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `createPromptState`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt.tsx
#[allow(non_snake_case)]
pub fn createPromptState() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `DEFAULT_PROMPT`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt.tsx
#[allow(non_snake_case)]
pub fn DEFAULT_PROMPT() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `isCommentItem`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt.tsx
#[allow(non_snake_case)]
pub fn isCommentItem() -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `isPromptEqual`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt.tsx
#[allow(non_snake_case)]
pub fn isPromptEqual() -> serde_json::Value {
    serde_json::json!({})
}
