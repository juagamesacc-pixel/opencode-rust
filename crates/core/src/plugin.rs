//! Rust port of `packages/core/src/plugin.ts`.

use serde::{Deserialize, Serialize};

pub const PLUGIN_SERVICE_KEY: &str = "@opencode/v2/Plugin";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct PluginError {
    pub message: String,
}
impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plugin error: {}", self.message)
    }
}

pub fn plugin_id_is_valid(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub mod agent;
pub mod command;
pub mod host;
pub mod internal;
pub mod layer_map_example;
pub mod models_dev;
pub mod promise;
pub mod provider;
pub mod skill;
pub mod variant;

// PROVISIONAL pending Effect Layer wiring — interface above is verbatim.
