// source: packages/plugin/src/v2 (directory barrel)
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2` directory (opencode v1.18.30).
//!
//! Source is a directory with `options.ts` + `effect/` + `promise/`.
//! Barrel re-exports via `v2/` are explicit per `effect/index.ts` and `promise/index.ts`.

pub mod effect;
pub mod options;
pub mod promise;
