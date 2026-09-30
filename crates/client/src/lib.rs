// source: packages/client/src/index.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/client/src/index.ts` (opencode v1.18.30).
//! Source 3 lines. 1:1 verbatim — see source comment below.
//! export * from "./generated/index"
//! export type { EventsSubscribeOutput as OpenCodeEvent } from "./generated/types"
//!

pub const SOURCE: &str = "packages/client/src/index.ts";

pub mod contract;
pub mod effect;
pub mod generated;
pub mod generated_effect;
