//! Rust port of `packages/app/src/utils/server-protocol.ts` (opencode v1.18.30).
//!
//! Source 35 lines: `ServerProtocol`, `detectServerProtocol` probe order
//! (`/global/health` legacy first, then `/api/health` V2 `pid` / transitional
//! V1 `healthy`). Fetch itself is PROVISIONAL; classification is verbatim.
//! Original file: `packages/app/src/utils/server-protocol.ts`

#![allow(dead_code)]

/// Mirrors `ServerProtocol`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerProtocol {
    V1,
    V2,
}

/// Mirrors the legacy `/global/health` classification.
pub fn classify_legacy_health(body: Option<&serde_json::Value>) -> Option<ServerProtocol> {
    let body = body?;
    if body.get("healthy").and_then(|v| v.as_bool()) == Some(true) {
        return Some(ServerProtocol::V1);
    }
    None
}

/// Mirrors the current `/api/health` classification.
pub fn classify_current_health(body: Option<&serde_json::Value>) -> Option<ServerProtocol> {
    let body = body?;
    if body.get("pid").and_then(|v| v.as_i64()).is_some() {
        return Some(ServerProtocol::V2);
    }
    if body.get("healthy").and_then(|v| v.as_bool()) == Some(true) {
        return Some(ServerProtocol::V1);
    }
    None
}

/// Mirrors `detectServerProtocol` fallback (defaults to `v2`).
pub fn detect_server_protocol(
    legacy: Option<&serde_json::Value>,
    current: Option<&serde_json::Value>,
) -> ServerProtocol {
    if let Some(protocol) = classify_legacy_health(legacy) {
        return protocol;
    }
    if let Some(protocol) = classify_current_health(current) {
        return protocol;
    }
    ServerProtocol::V2
}
