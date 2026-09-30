//! Rust port of `packages/app/src/utils/session.ts` (opencode v1.18.30).
//!
//! Source 38 lines: `normalizeSessionInfo`, `listAllSessions` pagination
//! (limit default `100`, cursor chaining). SDK client types are
//! PROVISIONAL; field mapping is verbatim.
//! Original file: `packages/app/src/utils/session.ts`

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors `listAllSessions` page size default (verbatim `100`).
pub const SESSION_LIST_LIMIT: u64 = 100;

/// Mirrors `normalizeSessionInfo(input)` V1→V2 field mapping (verbatim).
/// Source: `if (!("location" in input)) return input;` + `withTimestampedFallback` for title
/// and structured `revert` mapping.
pub fn normalize_session_info(input: &Value) -> Value {
    if input.get("location").is_none() {
        return input.clone();
    }
    let title = crate::utils::session_title::with_timestamped_fallback(
        input.get("title").and_then(|v| v.as_str()),
        input.get("parentID").and_then(|v| v.as_str()),
        input
            .get("time")
            .and_then(|v| v.get("created"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0),
    );
    let revert = input.get("revert").and_then(|v| {
        if v.is_null() {
            return None;
        }
        Some(serde_json::json!({
            "messageID": v.get("messageID"),
            "partID": v.get("partID"),
            "snapshot": v.get("snapshot"),
        }))
    });
    serde_json::json!({
        "id": input.get("id"),
        "slug": input.get("id"),
        "projectID": input.get("projectID"),
        "workspaceID": input.get("location").and_then(|v| v.get("workspaceID")),
        "directory": input.get("location").and_then(|v| v.get("directory")),
        "path": input.get("subpath"),
        "parentID": input.get("parentID"),
        "cost": input.get("cost"),
        "tokens": input.get("tokens"),
        "title": title,
        "agent": input.get("agent"),
        "model": input.get("model"),
        "version": "",
        "time": input.get("time"),
        "revert": revert.unwrap_or(serde_json::Value::Null),
    })
}

// PROVISIONAL: pending session client — mirrors `packages/app/src/utils/session.ts`.
/// Mirrors the cursor-page accumulator state.
#[derive(Debug, Default)]
pub struct SessionListAccumulator {
    pub collected: Vec<Value>,
}

impl SessionListAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_push(&mut self, page: Vec<Value>) {
        self.collected.extend(page);
    }

    pub fn transition_done(&self, page_len: usize, has_next: bool) -> bool {
        page_len == 0 || !has_next
    }
}
