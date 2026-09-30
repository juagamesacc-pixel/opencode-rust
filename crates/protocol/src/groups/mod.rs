//! Rust port of `packages/protocol/src/groups` barrel v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/error-strings/
//! status-codes/keys/defaults/ordering. Source is spec (`packages/protocol/src/groups/*`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.

pub mod agent;
pub mod command;
pub mod credential;
pub mod event;
pub mod fs;
pub mod health;
pub mod integration;
pub mod location;
pub mod message;
pub mod model;
pub mod permission;
pub mod project_copy;
pub mod provider;
pub mod pty;
pub mod question;
pub mod reference;
pub mod session;
pub mod skill;
