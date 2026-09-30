// source: packages/tui/src/util/model.ts (28 lines, v1.18.30)
// 1:1 port — provider/model lookup over serde shapes (SDK provider type).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Mirrors the SDK `Provider` shape actually read here: `id` plus a
/// `models` record keyed by model id. Unknown fields are preserved.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Provider {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub models: HashMap<String, Value>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

/// Mirrors `parse` — split `providerID/modelID` on the first slash.
pub fn parse(value: &str) -> (String, String) {
    match value.split_once('/') {
        Some((provider_id, model_id)) => (provider_id.to_string(), model_id.to_string()),
        None => (value.to_string(), String::new()),
    }
}

/// Mirrors `index` — provider list keyed by id.
pub fn index(list: &[Provider]) -> HashMap<&str, &Provider> {
    list.iter().map(|item| (item.id.as_str(), item)).collect()
}

/// Provider list as slice or prebuilt map, mirroring the TS overload.
pub enum ProviderList<'a> {
    Slice(&'a [Provider]),
    Map(&'a HashMap<String, Provider>),
}

/// Mirrors `get` — the model object for a provider/model pair.
pub fn get(list: ProviderList<'_>, provider_id: &str, model_id: &str) -> Option<Value> {
    let provider = match list {
        ProviderList::Map(map) => map.get(provider_id),
        ProviderList::Slice(items) => items.iter().find(|item| item.id == provider_id),
    }?;
    provider.models.get(model_id).cloned()
}

/// Mirrors `name` — model display name, falling back to the model id.
pub fn name(list: ProviderList<'_>, provider_id: &str, model_id: &str) -> String {
    get(list, provider_id, model_id)
        .and_then(|model| {
            model
                .get("name")
                .and_then(|n| n.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| model_id.to_string())
}
