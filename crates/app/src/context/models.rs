//! Rust port of `packages/app/src/context/models.tsx` (opencode v1.18.30).
//!
//! Source 174 lines. Exports: `ModelKey`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/models.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/models.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `ModelKey`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelKey {
    // PROVISIONAL: fields pending full port
}
