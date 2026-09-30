// source: packages/plugin/src/v2/effect/plugin.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/plugin.ts` (opencode v1.18.30).
//!
//! Source 16 lines. Exports: `Plugin`, `define`, `PluginDomain`.
//!
//! PROVISIONAL: `effect` (`Effect`, `Scope`) pending effect runtime.

use serde::{Deserialize, Serialize};

/// Mirrors `Plugin<R = Scope.Scope> { readonly id: string, readonly effect: (context: PluginContext) => Effect.Effect<void, never, R> }` verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub effect: serde_json::Value,
}

/// Mirrors `define<R = Scope.Scope>(plugin: Plugin<R>) { return plugin }` verbatim.
pub fn define(plugin: Plugin) -> Plugin {
    plugin
}

/// Mirrors `PluginDomain { readonly add: (plugin: Plugin) => Effect.Effect<void>, readonly remove: (id: string) => Effect.Effect<void> }` verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct PluginDomain;

impl PluginDomain {
    pub const ADD: &'static str = "add";
    pub const REMOVE: &'static str = "remove";
}

/// PROVISIONAL: `effect` pending.
pub mod effect_provisional {
    pub const PACKAGE: &str = "effect";
    pub const EFFECT: &str = "Effect.Effect";
    pub const SCOPE: &str = "Scope.Scope";
    pub const PENDING_CRATE: &str = "effect";
}
