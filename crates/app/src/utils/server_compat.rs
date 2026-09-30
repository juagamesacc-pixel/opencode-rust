//! Rust port of `packages/app/src/utils/server-compat.ts` (opencode v1.18.30).
//!
//! Source 518 lines: `CompatibleApi`/`CompatibleSessionApi` shapes,
//! `createCompatibleApi`, V1/V2 routing, `mime`, `sessionInfo`. Network
//! clients (`@opencode-ai/sdk/v2/client`, `@opencode-ai/client/promise`)
//! are PROVISIONAL; protocol selection + payload transforms are verbatim.
//! Original file: `packages/app/src/utils/server-compat.ts`

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors `ServerProtocol` selection (`v1` legacy vs `v2` current).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibleProtocol {
    V1,
    V2,
}

/// Mirrors the `mime(uri)` helper (verbatim fallback).
pub fn compat_mime(uri: &str) -> String {
    if let Some(rest) = uri.strip_prefix("data:") {
        let end = rest.find([';', ',']).unwrap_or(rest.len());
        let mime = &rest[..end];
        if !mime.is_empty() {
            return mime.to_string();
        }
    }
    "application/octet-stream".to_string()
}

/// Mirrors `sessionInfo(session)` V2 normalization (verbatim defaults).
pub fn compat_session_info(session: &Value) -> Value {
    let empty_tokens = serde_json::json!({"input": 0, "output": 0, "reasoning": 0, "cache": {"read": 0, "write": 0}});
    serde_json::json!({
        "id": session.get("id"),
        "parentID": session.get("parentID"),
        "projectID": session.get("projectID"),
        "agent": session.get("agent"),
        "model": session.get("model"),
        "cost": session.get("cost").and_then(|v| v.as_f64()).unwrap_or(0.0),
        "tokens": session.get("tokens").cloned().unwrap_or(empty_tokens),
        "time": session.get("time"),
        "title": session.get("title"),
    })
}

// PROVISIONAL: pending sdk/client protocol crates — mirrors `packages/app/src/utils/server-compat.ts`.
/// Mirrors `createCompatibleApi(input)` protocol routing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibleApiDescriptor {
    pub protocol: CompatibleProtocol,
    pub directory: Option<String>,
}

impl CompatibleApiDescriptor {
    pub fn new(protocol: CompatibleProtocol, directory: Option<&str>) -> Self {
        Self {
            protocol,
            directory: directory.map(str::to_string),
        }
    }

    pub fn update(&mut self, protocol: CompatibleProtocol) {
        self.protocol = protocol;
    }

    /// Mirrors the lazy `protocol === "v1" ? v1 : current` selection.
    pub fn transition_select(&self) -> &'static str {
        match self.protocol {
            CompatibleProtocol::V1 => "v1",
            CompatibleProtocol::V2 => "current",
        }
    }
}

/// Mirrors the `API method unavailable: {property}` error (verbatim).
pub fn api_method_unavailable(property: &str) -> String {
    format!("API method unavailable: {property}")
}

/// Mirrors the `API namespace unavailable: {property}` error (verbatim).
pub fn api_namespace_unavailable(property: &str) -> String {
    format!("API namespace unavailable: {property}")
}
