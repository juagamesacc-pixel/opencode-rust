#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `@opencode-ai/script` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/script`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! PROVISIONAL stubs: `Bun.file`/`$` shell/`process.*`/`semver` are Bun/runtime surfaces.
//! Represented as faithful descriptor constants/functions with verbatim error strings,
//! env keys, version ranges, channel logic. No behavior reinterpretation.
//! See `script.rs` for flagged inventory.

pub mod script;

// Barrel re-exports in source `src/` order:
pub use script::Script;
