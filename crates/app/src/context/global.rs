//! Rust port of `packages/app/src/context/global.tsx` (opencode v1.18.30).
//!
//! Source 162 lines. Exports: `ServerCtx`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/global.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `ServerCtx`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerCtx {
    // PROVISIONAL: fields pending full port
}
