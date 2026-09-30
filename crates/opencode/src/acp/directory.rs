// source: src/acp/directory.ts — exports: ModelOption, ModeOption,
// ModelVariants, DefaultModel, Snapshot, LoaderInterface, Interface, Loader,
// Service, modelKey, variants, build, loaderLayer, loaderNode, node, Directory
// PROVISIONAL pending core (provider, model, app-node, layer-node) + @/*:
// modelKey join, build() (Provider.sort order, variantsByModel, mode fallback
// modes[0] ?? default, commands localeCompare sort, optional defaultModel) verbatim.

use serde::{Deserialize, Serialize};

/// source: modelKey() — `${providerID}/${modelID}`. Verbatim.
pub fn model_key(provider_id: &str, model_id: &str) -> String {
    format!("{}/{}", provider_id, model_id)
}

/// source: Snapshot — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub directory: String,
    pub model_options: Vec<ModelOption>,
    pub available_modes: Vec<ModeOption>,
    pub default_mode_id: String,
    pub available_commands: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<DefaultModel>,
}

/// source: ModelOption — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelOption {
    pub provider_id: String,
    pub provider_name: String,
    pub model_id: String,
    pub model_name: String,
}

/// source: ModeOption — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeOption {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// source: DefaultModel — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultModel {
    pub provider_id: String,
    pub model_id: String,
}

/// source: defaultModeID fallback — modes includes default ? default :
/// modes[0] ?? default. Verbatim.
pub fn default_mode_id(modes: &[String], default: &str) -> String {
    if modes.iter().any(|m| m == default) {
        return default.to_string();
    }
    modes
        .first()
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

/// source: Service "@opencode/ACPDirectory" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/ACPDirectory";
/// source: Loader "@opencode/ACPDirectoryLoader" — verbatim service id.
pub const LOADER_ID: &str = "@opencode/ACPDirectoryLoader";
