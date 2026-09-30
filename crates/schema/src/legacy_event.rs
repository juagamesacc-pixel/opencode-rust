//! Port of `packages/schema/src/legacy-event.ts` (flat v1-legacy entrypoint).
//!
//! Source is exactly `export * from "./v1/legacy-event"` — this module is a
//! DISTINCT file that re-exports the isolated `v1/legacy_event` module (no
//! merge; shared type identity, as in source).

#![allow(non_snake_case)]

pub use crate::v1::legacy_event::*;
