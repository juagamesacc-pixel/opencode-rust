// source: packages/tui/src/util/tool-display.ts (13 lines, v1.18.30)
// 1:1 port — provider labels and the `structured` metadata gate verbatim.

#![allow(dead_code)]

use serde_json::{Map, Value};

/// Mirrors `webSearchProviderLabel`.
pub fn web_search_provider_label(provider: &Value) -> &'static str {
    match provider.as_str() {
        Some("parallel") => "Parallel Web Search",
        Some("exa") => "Exa Web Search",
        _ => "Web Search",
    }
}

/// Mirrors `toolDisplayMetadata` — non-object, pending, missing/non-object
/// `structured`, and array `structured` all yield an empty map.
pub fn tool_display_metadata(state: &Value) -> Map<String, Value> {
    let Some(map) = state.as_object() else { return Map::new() };
    if !map.contains_key("status") || map.get("status").and_then(|s| s.as_str()) == Some("pending") {
        return Map::new();
    }
    let Some(structured) = map.get("structured") else { return Map::new() };
    match structured {
        Value::Object(inner) => inner.clone(),
        _ => Map::new(),
    }
}