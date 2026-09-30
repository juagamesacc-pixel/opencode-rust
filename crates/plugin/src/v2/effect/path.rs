// source: packages/plugin/src/v2/effect/path.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/path.ts` (opencode v1.18.30).
//!
//! Source 8 lines. Exports: `Path` with home, data, cache, config, state, temp (all readonly string).

use serde::{Deserialize, Serialize};

/// Mirrors `Path` verbatim fields order: home, data, cache, config, state, temp.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub home: String,
    pub data: String,
    pub cache: String,
    pub config: String,
    pub state: String,
    pub temp: String,
}

pub const PATH_FIELDS: &[&str] = &["home", "data", "cache", "config", "state", "temp"];
