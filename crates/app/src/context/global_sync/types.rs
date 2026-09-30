//! Rust port of `packages/app/src/context/global-sync/types.ts` (opencode v1.18.30).
//!
//! Source 136 lines. Exports: `ProjectMeta`, `State`, `VcsCache`, `MetaCache`, `IconCache`, `ChildOptions`, `DirState`, `EvictPlan`, `DisposeCheck`, `MAX_DIR_STORES`, `DIR_IDLE_TTL_MS`, `SESSION_RECENT_WINDOW`, `SESSION_RECENT_LIMIT`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/global-sync/types.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/global-sync/types.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `ProjectMeta`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeta {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `State`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `VcsCache`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VcsCache {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `MetaCache`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaCache {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `IconCache`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconCache {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `ChildOptions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildOptions {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `DirState`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirState {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `EvictPlan`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvictPlan {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `DisposeCheck`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisposeCheck {
    // PROVISIONAL: fields pending full port
}

pub const MAX_DIR_STORES: i64 = 30;

pub const DIR_IDLE_TTL_MS: i64 = 20 * 60 * 1000;

pub const SESSION_RECENT_WINDOW: i64 = 4 * 60 * 60 * 1000;

pub const SESSION_RECENT_LIMIT: i64 = 50;
