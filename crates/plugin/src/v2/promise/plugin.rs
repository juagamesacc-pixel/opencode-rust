// source: packages/plugin/src/v2/promise/plugin.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/plugin.ts` (opencode v1.18.30).
//!
//! Source 15 lines. Exports: `Plugin { id, setup }`, `define`, `PluginDomain`.

use serde::{Deserialize, Serialize};

/// Mirrors `Plugin { readonly id: string, readonly setup: (context: PluginContext) => Promise<void> | void }` verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub setup: serde_json::Value,
}

/// Mirrors `define(plugin: Plugin) { return plugin }` verbatim.
pub fn define(plugin: Plugin) -> Plugin {
    plugin
}

/// Mirrors `PluginDomain { readonly add: (plugin: Plugin) => Promise<void>, readonly remove: (id: string) => Promise<void> }` verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct PluginDomain;

impl PluginDomain {
    pub const ADD: &'static str = "add";
    pub const REMOVE: &'static str = "remove";
}
