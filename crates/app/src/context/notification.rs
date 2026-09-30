//! Rust port of `packages/app/src/context/notification.tsx` (opencode v1.18.30).
//!
//! Source 484 lines. Exports: `Notification`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/notification.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/notification.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `Notification`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    // PROVISIONAL: fields pending full port
}
