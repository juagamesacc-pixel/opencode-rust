//! Rust port of `packages/app/src/utils/diffs.ts` (opencode v1.18.30).
//!
//! Source 50 lines: `diff` type guard, `diffs(value)`, `message(value)`.
//!
//! 1:1 notes:
//! - `Diff` models `FileDiffInfo | SnapshotFileDiff | VcsFileDiff`
//!   (`@opencode-ai/sdk/v2`, `@opencode-ai/client/promise`) locally;
//!   `additions`/`deletions` are parsed via `as_i64` (SDK numbers are JSON
//!   integers).
//! - `Message` is modelled as `serde_json::Value` (the source spreads and
//!   rebuilds it); the `next === raw.diffs` reference check is always false for
//!   a freshly built array, so object summaries are always rebuilt.
//! - `Object.values(value)` order in the source is JS insertion order; here it
//!   follows `serde_json`'s map order (sorted) — noted for multi-key objects.
//! - Original file: `packages/app/src/utils/diffs.ts`

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors `Diff`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Diff {
    pub file: String,
    pub patch: String,
    pub additions: i64,
    pub deletions: i64,
    pub status: Option<DiffStatus>,
}

/// Mirrors the `status?: "added" | "deleted" | "modified"` union.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum DiffStatus {
    Added,
    Deleted,
    Modified,
}

/// Mirrors the `diff(value): value is Diff` guard.
pub fn is_diff(value: &Value) -> bool {
    if !value.is_object() {
        return false;
    }
    if !match value.get("file").and_then(|f| f.as_str()) {
        Some(_) => true,
        None => return false,
    } {
        return false;
    }
    if value.get("patch").and_then(|p| p.as_str()).is_none() {
        return false;
    }
    if value.get("additions").and_then(|n| n.as_i64()).is_none() {
        return false;
    }
    if value.get("deletions").and_then(|n| n.as_i64()).is_none() {
        return false;
    }
    matches!(
        value.get("status").and_then(|s| s.as_str()),
        None | Some("added") | Some("deleted") | Some("modified")
    )
}

fn parse_diff(value: &Value) -> Diff {
    Diff {
        file: value["file"].as_str().unwrap_or_default().to_string(),
        patch: value["patch"].as_str().unwrap_or_default().to_string(),
        additions: value["additions"].as_i64().unwrap_or_default(),
        deletions: value["deletions"].as_i64().unwrap_or_default(),
        status: value
            .get("status")
            .and_then(|s| s.as_str())
            .map(|s| match s {
                "added" => DiffStatus::Added,
                "deleted" => DiffStatus::Deleted,
                _ => DiffStatus::Modified,
            }),
    }
}

fn is_object(value: &Value) -> bool {
    value.is_object()
}

/// Mirrors `diffs(value): Diff[]`.
pub fn diffs(value: &Value) -> Vec<Diff> {
    if let Some(array) = value.as_array() {
        return array
            .iter()
            .filter(|item| is_diff(item))
            .map(parse_diff)
            .collect();
    }
    if is_diff(value) {
        return vec![parse_diff(value)];
    }
    if !is_object(value) {
        return Vec::new();
    }
    value
        .as_object()
        .expect("checked is_object")
        .values()
        .filter(|item| is_diff(item))
        .map(parse_diff)
        .collect()
}

/// Mirrors `message(value: Message): Message`.
pub fn message(value: Value) -> Value {
    if value.get("role").and_then(|r| r.as_str()) != Some("user") {
        return value;
    }
    let raw = match value.get("summary").cloned() {
        Some(raw) => raw,
        None => return value,
    };
    if !is_object(&raw) {
        let mut next = value;
        next["summary"] = Value::Null;
        return next;
    }
    let title = raw
        .get("title")
        .and_then(|t| t.as_str())
        .map(str::to_string);
    let body = raw.get("body").and_then(|b| b.as_str()).map(str::to_string);
    let next_diffs = diffs(raw.get("diffs").unwrap_or(&Value::Null));
    // TS: `next === raw.diffs` compares a fresh array against the stored value
    // and is always false, so the summary is always rebuilt for object raw.
    let mut summary = serde_json::Map::new();
    if let Some(title) = title {
        summary.insert("title".to_string(), Value::String(title));
    }
    if let Some(body) = body {
        summary.insert("body".to_string(), Value::String(body));
    }
    summary.insert(
        "diffs".to_string(),
        serde_json::to_value(&next_diffs).unwrap_or(Value::Null),
    );
    let mut next = value;
    next["summary"] = Value::Object(summary);
    next
}
