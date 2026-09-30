// source: packages/tui/src/util/record.ts (3 lines, v1.18.30)
// 1:1 port — plain-object guard over serde_json::Value.

#![allow(dead_code)]

use serde_json::{Map, Value};

/// Mirrors `isRecord` — non-null object, not an array.
pub fn is_record(value: &Value) -> bool {
    matches!(value, Value::Object(_))
}

pub fn as_record(value: &Value) -> Option<&Map<String, Value>> {
    if let Value::Object(map) = value {
        Some(map)
    } else {
        None
    }
}
