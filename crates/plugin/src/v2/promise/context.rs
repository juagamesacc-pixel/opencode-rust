// source: packages/plugin/src/v2/promise/context.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/context.ts` (opencode v1.18.30).
//!
//! Source 22 lines. Exports: `PluginContext` Promise variant (same fields as effect but Promise).

/// Mirrors `PluginContext` Promise variant field names verbatim order: options, agent, aisdk, catalog, command, integration, plugin, reference, skill.
pub const PLUGIN_CONTEXT_FIELDS: &[&str] = &[
    "options",
    "agent",
    "aisdk",
    "catalog",
    "command",
    "integration",
    "plugin",
    "reference",
    "skill",
];

/// Mirrors `PluginContext` Promise descriptor.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginContext {
    pub options: serde_json::Value,
    pub agent: serde_json::Value,
    pub aisdk: serde_json::Value,
    pub catalog: serde_json::Value,
    pub command: serde_json::Value,
    pub integration: serde_json::Value,
    pub plugin: serde_json::Value,
    pub reference: serde_json::Value,
    pub skill: serde_json::Value,
}
