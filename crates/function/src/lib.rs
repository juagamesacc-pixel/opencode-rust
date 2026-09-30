#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `@opencode-ai/function` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/function`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! PROVISIONAL stubs: imports from `cloudflare:workers`, `hono`, `sst`, `jose`,
//! `@octokit/*`, `node:crypto` are runtime-provided Workers/Bun surfaces. They
//! are represented as faithful local descriptor constants/types (`api::workers_provisional`,
//! `api::hono_provisional`, `api::sst_provisional`, etc.) with the same IDs/keys/strings.
//! Each stub is flagged PROVISIONAL pending a dedicated runtime crate. No behavior
//! reinterpretation — stubs are minimal equivalents that preserve observable semantics
//! (error strings, status codes, header names, route paths, defaults). See `api.rs`
//! for the full flagged inventory.

pub mod api;
pub mod github;

// Barrel re-exports in source `src/` order (mirrors TS exports):
pub use api::{Api, SyncServer};
pub use github::parse_repository_claim;
