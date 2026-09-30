#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `@opencode-ai/slack` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/slack`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! PROVISIONAL stubs: `@slack/bolt` `App` and `@opencode-ai/sdk` `createOpencode` are
//! host-provided runtime surfaces. Represented as faithful local descriptor constants/types
//! (`slack_provisional`, `sdk_provisional`) with verbatim env keys, event names, command
//! strings, defaults and error strings. Each stub is flagged PROVISIONAL pending the
//! upstream crate. No behavior reinterpretation. See `bot.rs` for inventory.

pub mod bot;

// Barrel re-exports in source `src/` order:
pub use bot::{SlackBot, SlackConfig};
