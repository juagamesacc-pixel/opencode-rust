#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `@opencode-ai/sdk-next` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/sdk-next`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! PROVISIONAL stubs: imports from `@opencode-ai/client/effect`, `@opencode-ai/core/*`,
//! `@opencode-ai/server/routes`, `effect/*`, `effect/unstable/http` are host runtimes
//! (pending `crates/client`, `crates/core`, `crates/server`). Represented as faithful
//! local descriptor constants/types (`opencode::core_provisional`, `opencode::client_provisional`,
//! `opencode::server_provisional`, `opencode::effect_provisional`) with verbatim service IDs,
//! method names, error tags, base URLs, layer IDs. Each stub is flagged PROVISIONAL pending
//! the upstream crate. No behavior reinterpretation. See `opencode.rs` + `tool.rs` for inventory.

pub mod opencode;
pub mod tool;

// Barrel re-exports in source `src/index.ts` order (mirrors TS `export * as ...` + re-exports):
pub use opencode::{create, layer, Interface, Service, BASE_URL, SERVICE_ID};
pub use tool::{AnyTool, Failure, RegistrationError, Tool};
