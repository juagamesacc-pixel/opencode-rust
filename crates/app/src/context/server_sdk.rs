//! Rust port of `packages/app/src/context/server-sdk.tsx` (opencode v1.18.30).
//!
//! Source 445 lines. Exports: `ServerEvent`, `adaptServerEvent`, `enqueueServerEvent`, `coalesceServerEvents`, `resumeStreamAfterPageShow`, `ServerSDK`, `createServerSdkContext`, `useServerProtocol`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/server-sdk.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/server-sdk.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `ServerEvent`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerEvent {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `adaptServerEvent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-sdk.tsx
#[allow(non_snake_case)]
pub fn adaptServerEvent(/* event: OpenCodeEvent */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `enqueueServerEvent`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-sdk.tsx
#[allow(non_snake_case)]
pub fn enqueueServerEvent(/* queue: QueuedServerEvent[], event: QueuedServerEvent */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `coalesceServerEvents`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-sdk.tsx
#[allow(non_snake_case)]
pub fn coalesceServerEvents(/* events: QueuedServerEvent[] */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `resumeStreamAfterPageShow`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-sdk.tsx
#[allow(non_snake_case)]
pub fn resumeStreamAfterPageShow(/* event: PageTransitionEvent, start: ( */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `ServerSDK`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerSDK {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `createServerSdkContext`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-sdk.tsx
#[allow(non_snake_case)]
pub fn createServerSdkContext(/* server: ServerConnection.Any, scope: ServerScope */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `useServerProtocol`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/server-sdk.tsx
#[allow(non_snake_case)]
pub fn useServerProtocol() -> serde_json::Value {
    serde_json::json!({})
}
