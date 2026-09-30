//! Rust port of `packages/app/src/context/sdk.tsx` (opencode v1.18.30).
//!
//! Source 18 lines. Exports: `DirectorySDK`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/sdk.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/sdk.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `DirectorySDK`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectorySDK {
    // PROVISIONAL: fields pending full port
}
