//! Rust port of `packages/app/src/context/platform.tsx` (opencode v1.18.30).
//!
//! Source 144 lines. Exports: `FatalRendererErrorLog`, `Platform`, `DisplayBackend`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/platform.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/platform.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `FatalRendererErrorLog`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FatalRendererErrorLog {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Platform`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `DisplayBackend`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayBackend {
    // PROVISIONAL: fields pending full port
}
