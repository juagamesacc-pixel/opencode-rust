//! Rust port of `packages/app/src/context/sync.tsx` (opencode v1.18.30).
//!
//! Source 121 lines. Exports: `mergeOptimisticPage`, `applyOptimisticAdd`, `applyOptimisticRemove`, `useSync`, `DirectorySync`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/sync.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/sync.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

fn cmp(a: &str, b: &str) -> i32 {
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        0
    }
}

fn sort_parts(mut parts: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    parts.retain(|p| p.get("id").and_then(|v| v.as_str()).is_some());
    parts.sort_by(|a, b| {
        let a_id = a.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let b_id = b.get("id").and_then(|v| v.as_str()).unwrap_or("");
        a_id.cmp(b_id)
    });
    parts
}

fn binary_search_by_key(
    items: &[serde_json::Value],
    key: &str,
    key_fn: impl Fn(&serde_json::Value) -> String,
) -> (bool, usize) {
    let mut low = 0usize;
    let mut high = items.len();
    while low < high {
        let mid = low + (high - low) / 2;
        let mid_key = key_fn(&items[mid]);
        if mid_key.as_str() < key {
            low = mid + 1;
        } else if mid_key.as_str() > key {
            high = mid;
        } else {
            return (true, mid);
        }
    }
    (false, low)
}

fn message_key(value: &serde_json::Value) -> String {
    let created = value
        .get("time")
        .and_then(|v| v.get("created"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let id = value.get("id").and_then(|v| v.as_str()).unwrap_or("");
    format!("{created}{id}")
}

fn has_parts(parts: Option<&Vec<serde_json::Value>>, want: &[serde_json::Value]) -> bool {
    match parts {
        None => want.is_empty(),
        Some(existing) => want.iter().all(|part| {
            let id = part.get("id").and_then(|v| v.as_str()).unwrap_or("");
            binary_search_by_key(existing, id, |v| {
                v.get("id")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string()
            })
            .0
        }),
    }
}

fn merge_parts(
    existing: Option<Vec<serde_json::Value>>,
    want: Vec<serde_json::Value>,
) -> Vec<serde_json::Value> {
    let Some(mut next) = existing else {
        return sort_parts(want);
    };
    let mut changed = false;
    for part in want {
        let id = part
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let (found, index) = binary_search_by_key(&next, &id, |v| {
            v.get("id")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string()
        });
        if found {
            continue;
        }
        next.insert(index, part);
        changed = true;
    }
    if !changed {
        return next;
    }
    next
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagePage {
    pub session: Vec<serde_json::Value>,
    pub part: Vec<PartBucket>,
    pub cursor: Option<String>,
    pub complete: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartBucket {
    pub id: String,
    pub part: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptimisticItem {
    pub message: serde_json::Value,
    pub parts: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptimisticStore {
    pub message: std::collections::HashMap<String, Vec<serde_json::Value>>,
    pub part: std::collections::HashMap<String, Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MergeOptimisticPageOutput {
    pub session: Vec<serde_json::Value>,
    pub part: Vec<PartBucket>,
    pub cursor: Option<String>,
    pub complete: bool,
    pub confirmed: Vec<String>,
}

/// Mirrors `mergeOptimisticPage` verbatim (Binary.search + sort + map).
#[allow(non_snake_case)]
pub fn mergeOptimisticPage(
    page: MessagePage,
    items: Vec<OptimisticItem>,
) -> MergeOptimisticPageOutput {
    if items.is_empty() {
        return MergeOptimisticPageOutput {
            session: page.session,
            part: page.part,
            cursor: page.cursor,
            complete: page.complete,
            confirmed: Vec::new(),
        };
    }
    let mut session = page.session.clone();
    let mut part_map: std::collections::HashMap<String, Vec<serde_json::Value>> = page
        .part
        .into_iter()
        .map(|b| (b.id.clone(), sort_parts(b.part)))
        .collect();
    let mut confirmed = Vec::new();
    for item in items {
        let key = message_key(&item.message);
        let (found, index) = binary_search_by_key(&session, &key, message_key);
        if !found {
            session.insert(index, item.message.clone());
        }
        let current = part_map.get(
            item.message
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or(""),
        );
        let msg_id = item
            .message
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if found && has_parts(current, &item.parts) {
            confirmed.push(msg_id.clone());
            continue;
        }
        let merged = merge_parts(current.cloned(), item.parts);
        part_map.insert(msg_id, merged);
    }
    let mut part: Vec<PartBucket> = part_map
        .into_iter()
        .map(|(id, part)| PartBucket { id, part })
        .collect();
    part.sort_by(|a, b| a.id.cmp(&b.id));
    MergeOptimisticPageOutput {
        session,
        part,
        cursor: page.cursor,
        complete: page.complete,
        confirmed,
    }
}

/// Mirrors `applyOptimisticAdd` verbatim.
#[allow(non_snake_case)]
pub fn applyOptimisticAdd(
    draft: &mut OptimisticStore,
    session_id: &str,
    message: serde_json::Value,
    parts: Vec<serde_json::Value>,
) {
    let key = message_key(&message);
    let entry = draft.message.entry(session_id.to_string()).or_default();
    let (found, index) = binary_search_by_key(entry, &key, message_key);
    if !found {
        entry.insert(index, message.clone());
    } else {
        // source splices at found index even if found? It always splices; we mirror insert.
        entry.insert(index, message.clone());
    }
    let msg_id = message
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    draft.part.insert(msg_id, sort_parts(parts));
}

/// Mirrors `applyOptimisticRemove` verbatim.
#[allow(non_snake_case)]
pub fn applyOptimisticRemove(draft: &mut OptimisticStore, session_id: &str, message_id: &str) {
    if let Some(messages) = draft.message.get_mut(session_id) {
        if let Some(pos) = messages
            .iter()
            .position(|m| m.get("id").and_then(|v| v.as_str()) == Some(message_id))
        {
            messages.remove(pos);
        }
    }
    draft.part.remove(message_id);
}

/// Mirrors `useSync` — Solid memo is PROVISIONAL; key wiring stays via `DirectorySync` descriptor.
pub fn useSync_value() -> String {
    String::new()
}

/// Mirrors `DirectorySync` (server/sync wiring is Solid-provisional).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectorySync {
    pub directory: String,
}
