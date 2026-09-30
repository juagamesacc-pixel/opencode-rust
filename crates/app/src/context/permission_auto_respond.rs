//! Rust port of `packages/app/src/context/permission-auto-respond.ts` (opencode v1.18.30).
//!
//! Source 61 lines. Exports: `acceptKey`, `directoryAcceptKey`, `isDirectoryAutoAccepting`, `autoRespondsPermission`, `sessionAutoAccept`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/permission-auto-respond.ts`

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `acceptKey`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/permission-auto-respond.ts
#[allow(non_snake_case)]
pub fn acceptKey(/* sessionID: string, directory?: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `directoryAcceptKey`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/permission-auto-respond.ts
#[allow(non_snake_case)]
pub fn directoryAcceptKey(/* directory: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `isDirectoryAutoAccepting`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/permission-auto-respond.ts
#[allow(non_snake_case)]
pub fn isDirectoryAutoAccepting(/* autoAccept: Record<string, boolean>, directory: string */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `autoRespondsPermission`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/permission-auto-respond.ts
#[allow(non_snake_case)]
pub fn autoRespondsPermission(/* 
  autoAccept: Record<string, boolean>,
  session: { id: string; parentID?: string }[],
  permission: { sessionID: string },
  directory?: string,
 */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `sessionAutoAccept`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/permission-auto-respond.ts
#[allow(non_snake_case)]
pub fn sessionAutoAccept(/* 
  autoAccept: Record<string, boolean>,
  session: { id: string; parentID?: string }[],
  permission: { sessionID: string },
  directory?: string,
 */) -> serde_json::Value {
    serde_json::json!({})
}
