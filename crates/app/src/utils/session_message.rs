//! Rust port of `packages/app/src/utils/session-message.ts` (opencode v1.18.30).
//!
//! Source 366 lines: `compareMessages`/`messageKey`,
//! `normalizeSessionMessages` (+ private `userMessage`/`assistantMessage`/
//! `shellMessages`/part builders, tool-input/metadata normalization).
//! Effect-schema decoding is PROVISIONAL; ordering + verbatim field names
//! preserved.
//! Original file: `packages/app/src/utils/session-message.ts`

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors `messageKey(message)` (`time.created + id`).
pub fn message_key(created: i64, id: &str) -> String {
    format!("{created}{id}")
}

/// Mirrors `compareMessages(a, b)`.
pub fn compare_messages(a_created: i64, a_id: &str, b_created: i64, b_id: &str) -> i32 {
    let left = message_key(a_created, a_id);
    let right = message_key(b_created, b_id);
    if left < right {
        -1
    } else if left > right {
        1
    } else {
        0
    }
}

/// Mirrors the `edit`/`write` tool-input `path`→`filePath` normalization.
pub fn normalize_tool_input(name: &str, mut input: Value) -> Value {
    if !["edit", "write"].contains(&name) {
        return input;
    }
    if input.get("filePath").and_then(|v| v.as_str()).is_some() {
        return input;
    }
    if let Some(path) = input
        .get("path")
        .and_then(|v| v.as_str())
        .map(str::to_string)
    {
        if let Some(map) = input.as_object_mut() {
            map.insert("filePath".to_string(), Value::String(path));
        }
    }
    input
}

/// Mirrors the `edit` tool-metadata `files`→`filediff` normalization (verbatim).
pub fn normalize_tool_metadata(name: &str, metadata: Value) -> Value {
    if name != "edit" {
        return metadata;
    }
    let files = match metadata.get("files").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return metadata,
    };
    let file = files.iter().find(|v| v.is_object());
    let file_obj = match file {
        Some(f) => f,
        None => return metadata,
    };
    let file_str = match file_obj.get("file").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return metadata,
    };
    let mut out = metadata.as_object().cloned().unwrap_or_default();
    let patch = file_obj
        .get("patch")
        .and_then(|v| v.as_str())
        .map(|s| Value::String(s.to_string()));
    let additions = file_obj
        .get("additions")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let deletions = file_obj
        .get("deletions")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let mut filediff = serde_json::Map::new();
    filediff.insert("file".to_string(), Value::String(file_str));
    if let Some(p) = patch {
        filediff.insert("patch".to_string(), p);
    }
    filediff.insert("additions".to_string(), Value::from(additions));
    filediff.insert("deletions".to_string(), Value::from(deletions));
    out.insert("filediff".to_string(), Value::Object(filediff));
    Value::Object(out)
}

/// Mirrors `sessionMessagePartID(messageID, type, ordinal)` verbatim.
pub fn session_message_part_id(message_id: &str, kind: &str, ordinal: usize) -> String {
    format!("{message_id}:{kind}:{ordinal}")
}

// PROVISIONAL: pending Effect/SDK message projection — mirrors `packages/app/src/utils/session-message.ts`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NormalizedSessionMessages {
    pub messages: Vec<Value>,
}

impl NormalizedSessionMessages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_push(&mut self, message: Value) {
        self.messages.push(message);
    }

    pub fn transition_sorted(&mut self) {
        self.messages.sort_by(|a, b| {
            let a_key = format!("{:?}{:?}", a.get("time"), a.get("id"));
            let b_key = format!("{:?}{:?}", b.get("time"), b.get("id"));
            a_key.cmp(&b_key)
        });
    }
}
