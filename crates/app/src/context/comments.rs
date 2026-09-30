//! Rust port of `packages/app/src/context/comments.tsx` (opencode v1.18.30).
//!
//! Source 262 lines. Exports: `LineComment`, `createCommentSessionForTest`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/comments.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/comments.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `LineComment`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineComment {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `createCommentSessionForTest`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/comments.tsx
#[allow(non_snake_case)]
pub fn createCommentSessionForTest(/* comments: Record<string, LineComment[]> = {} */
) -> serde_json::Value {
    serde_json::json!({})
}
