#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `@opencode-ai/effect-sqlite-node` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/effect-sqlite-node`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! PROVISIONAL stubs: `node:sqlite` `DatabaseSync` + `effect/*` (`Context`/`Effect`/`Fiber`/
//! `Layer`/`Scope`/`Semaphore`/`Stream`/`Reactivity`/`SqlClient`/`SqlConnection`/`SqlError`/
//! `Statement`) are runtime surfaces. Represented as faithful local descriptor constants/types
//! (`client::effect_provisional`, `client::node_sqlite_provisional`) with verbatim service IDs,
//! error strings, span keys, pragma strings. Each stub is flagged PROVISIONAL pending the
//! upstream `effect`/`node:sqlite` crate. No behavior reinterpretation — stubs are minimal
//! equivalents preserving observable semantics. See `client.rs` for inventory.

pub mod client;

// Barrel re-exports in source `src/` order (mirrors TS `export * as NodeSqliteClient` + named exports):
pub use client::{NodeSqliteClient, SqliteClient, SqliteClientConfig, TypeId, ATTR_DB_SYSTEM_NAME};
