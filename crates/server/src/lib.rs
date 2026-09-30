#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `@opencode-ai/server` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/server`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! Core wiring: imports from `@opencode-ai/core/*` now resolve to `crates/core`
//! where the API is ported (Location/Session/Pty pure types and service IDs).
//! `core_provisional` retains only unported runtime surfaces (Ticket/Protocol/
//! Database/Event/Catalog) flagged PROVISIONAL with reason. Pure branches
//! (error strings, status codes, cursor/CORS) are verbatim from source.

pub mod api;
pub mod auth;
pub mod core_provisional;
pub mod cors;
pub mod handlers;
pub mod location;
pub mod pty_environment;
pub mod routes;

pub mod handlers_impl;
pub mod middleware;

// Barrel re-exports in source `src/` order (mirrors TS exports):
pub use api::Api;
pub use auth::ServerAuth;
pub use cors::{CorsConfig, CorsOptions};
pub use location::{response_with_location, LocationMiddleware};
pub use pty_environment::PtyEnvironment;
