// source: packages/plugin/src/v2/effect/npm.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/npm.ts` (opencode v1.18.30).
//!
//! Source 11 lines. Exports: `Npm` with `add(pkg: string): Effect<{directory, entrypoint?}, unknown>`.
//!
//! PROVISIONAL: `effect` pending.

/// Mirrors `Npm` method verbatim.
pub const NPM_METHODS: &[&str] = &["add"];

/// Mirrors `Npm` descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct Npm;

impl Npm {
    pub const ADD: &'static str = "add";
}

/// PROVISIONAL: `effect` pending.
pub mod effect_provisional {
    pub const PACKAGE: &str = "effect";
    pub const EFFECT: &str = "Effect.Effect";
    pub const PENDING_CRATE: &str = "effect";
}
