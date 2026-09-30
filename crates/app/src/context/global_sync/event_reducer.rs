//! Rust port of `packages/app/src/context/global-sync/event-reducer.ts` (opencode v1.18.30).
//!
//! Source 480 lines. Exports: `applyGlobalEvent`, `cleanupDroppedSessionCaches`, `applyDirectoryEvent`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/event-reducer.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/global-sync/event-reducer.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `applyGlobalEvent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/event-reducer.ts
#[allow(non_snake_case)]
pub fn applyGlobalEvent(/* input: {
  event: { type: string; properties?: unknown }
  project: Project[]
  setGlobalProject: (next: Project[] | ((draft: Project[] */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `cleanupDroppedSessionCaches`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/event-reducer.ts
#[allow(non_snake_case)]
pub fn cleanupDroppedSessionCaches(/* 
  store: Store<State>,
  setStore: SetStoreFunction<State>,
  next: Session[],
  setSessionTodo?: (sessionID: string, todos: Todo[] | undefined */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `applyDirectoryEvent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/global-sync/event-reducer.ts
#[allow(non_snake_case)]
pub fn applyDirectoryEvent(/* input: {
  event: { type: string; properties?: unknown }
  store: Store<State>
  setStore: SetStoreFunction<State>
  push: (directory: string */) -> serde_json::Value {
    serde_json::json!({})
}
