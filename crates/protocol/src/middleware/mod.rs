//! Rust port of `packages/protocol/src/middleware` barrel v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/error-strings/
//! status-codes/keys/defaults/ordering. Source is spec (`packages/protocol/src/middleware/*`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.

pub mod authorization;
pub mod schema_error;
