#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `packages/containers` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/containers`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! PROVISIONAL stubs: Docker/buildx surface (`docker build`/`docker buildx build`)
//! is host-provided. Represented as faithful descriptor constants with verbatim
//! registry/tag/platform/image/file/arg strings. No behavior reinterpretation.
//! See `build.rs` and `images.rs` for flagged inventory.

pub mod build;
pub mod images;

// Barrel re-exports in source `packages/containers` order:
pub use build::{BuildConfig, BuildScript};
pub use images::{Dockerfile, Image};
