//! Rust port of `packages/core/src/config/plugin.ts` + `packages/core/src/config/plugin` barrel.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

// ---- plugin.ts types ----
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub package: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Plugin {
    Str(String),
    Entry(Entry),
}

pub type Plugins = Vec<Plugin>;

// ---- plugin/* submodules ----
pub mod agent;
pub mod command;
pub mod external;
pub mod provider;
pub mod reference;
pub mod skill;
