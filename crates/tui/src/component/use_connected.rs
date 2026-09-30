// source: packages/tui/src/component/use-connected.tsx (12 lines, v1.18.30)
// 1:1 port — any non-`opencode` provider, or an `opencode` model with
// non-zero input cost, counts as connected.

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors `useConnected` over the sync provider list.
pub fn is_connected(providers: &[Value]) -> bool {
    providers.iter().any(|provider| {
        if provider.get("id").and_then(|v| v.as_str()) != Some("opencode") {
            return true;
        }
        provider
            .get("models")
            .and_then(|m| m.as_object())
            .map(|models| {
                models.values().any(|model| {
                    model
                        .get("cost")
                        .and_then(|c| c.get("input"))
                        .and_then(|v| v.as_f64())
                        != Some(0.0)
                        && model.get("cost").and_then(|c| c.get("input")).is_some()
                })
            })
            .unwrap_or(false)
    })
}
