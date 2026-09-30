// source: packages/plugin/src/v2/effect/index.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/index.ts` (opencode v1.18.30).
//!
//! Source 3 lines: `export type { PluginContext } from "./context.js"`, `export { define } from "./plugin.js"`, `export type { Plugin } from "./plugin.js"`.

pub mod agent;
pub mod aisdk;
pub mod catalog;
pub mod command;
pub mod context;
pub mod event;
pub mod filesystem;
pub mod integration;
pub mod location;
pub mod npm;
pub mod path;
pub mod plugin;
pub mod reference;
pub mod registration;
pub mod skill;

pub use context::PluginContext;
pub use plugin::{define, Plugin};
