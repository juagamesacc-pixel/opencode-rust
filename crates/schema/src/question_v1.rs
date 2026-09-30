//! Port of `packages/schema/src/question-v1.ts` (flat v1 entrypoint).
//!
//! Source is exactly `export * from "./v1/question"` — this module is a
//! DISTINCT file that re-exports the isolated `v1/question` module (no merge;
//! shared type identity, as in source).

#![allow(non_snake_case)]

pub use crate::v1::question::*;
