//! Rust port of `packages/app/src/context/server-session.ts` (opencode v1.18.30).
//!
//! Source 1428 lines. Exports: `createServerSession`, `ServerSession`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/server-session.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/server-session.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `createServerSession`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-session.ts
#[allow(non_snake_case)]
pub fn createServerSession(/* 
  client: OpencodeClient,
  sessionApiOrOptions?: SessionApi | ServerSessionOptions,
  messageApi?: MessageApi,
  currentOptions?: ServerSessionOptions,
 */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `ServerSession`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerSession {
    // PROVISIONAL: fields pending full port
}
