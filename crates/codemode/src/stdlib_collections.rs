//! Port of `src/stdlib/collections.ts`.
//!
//! Method-name tables + `spreadItems`.

use crate::values::{RtValue, SandboxMap, SandboxSet, SandboxURLSearchParams};

/// Array methods, verbatim. Mirrors `arrayMethods`.
pub const ARRAY_METHODS: &[&str] = &[
    "map",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "some",
    "every",
    "includes",
    "join",
    "reduce",
    "reduceRight",
    "flatMap",
    "forEach",
    "sort",
    "toSorted",
    "slice",
    "concat",
    "indexOf",
    "lastIndexOf",
    "at",
    "flat",
    "reverse",
    "toReversed",
    "with",
    "push",
    "pop",
    "shift",
    "unshift",
    "splice",
    "fill",
    "copyWithin",
    "keys",
    "values",
    "entries",
];

/// Map methods, verbatim. Mirrors `mapMethods`.
pub const MAP_METHODS: &[&str] = &[
    "get", "set", "has", "delete", "clear", "forEach", "keys", "values", "entries",
];

/// Set methods, verbatim. Mirrors `setMethods`.
pub const SET_METHODS: &[&str] = &[
    "add", "has", "delete", "clear", "forEach", "keys", "values", "entries",
];

/// Mirrors `arrayMethods.has(name)`.
pub fn is_array_method(name: &str) -> bool {
    ARRAY_METHODS.contains(&name)
}

/// Mirrors `mapMethods.has(name)`.
pub fn is_map_method(name: &str) -> bool {
    MAP_METHODS.contains(&name)
}

/// Mirrors `setMethods.has(name)`.
pub fn is_set_method(name: &str) -> bool {
    SET_METHODS.contains(&name)
}

/// Spreadable items of an iterable value. Mirrors `spreadItems(value)`:
/// arrays spread as-is, strings spread by char, Maps spread as `[k, v]`
/// pairs, Sets spread members, URLSearchParams spread `[k, v]` pairs.
pub fn spread_items_of_json(value: &serde_json::Value) -> Option<Vec<serde_json::Value>> {
    match value {
        serde_json::Value::Array(items) => Some(items.clone()),
        serde_json::Value::String(s) => Some(
            s.chars()
                .map(|c| serde_json::Value::String(c.to_string()))
                .collect(),
        ),
        _ => None,
    }
}

/// Spread items of a sandbox Map (as `[k, v]` pairs of live values).
pub fn spread_sandbox_map(map: &SandboxMap) -> Vec<RtValue> {
    map.entries
        .iter()
        .map(|(k, v)| RtValue::Array(crate::values::RtArray::new(vec![k.clone(), v.clone()])))
        .collect()
}

/// Spread members of a sandbox Set (live values).
pub fn spread_sandbox_set(set: &SandboxSet) -> Vec<RtValue> {
    set.members.clone()
}

/// Spread entries of sandbox URLSearchParams (as `[k, v]` pairs).
pub fn spread_sandbox_params(params: &SandboxURLSearchParams) -> Vec<RtValue> {
    params
        .pairs
        .iter()
        .map(|(k, v)| {
            RtValue::Array(crate::values::RtArray::new(vec![
                RtValue::Str(k.clone()),
                RtValue::Str(v.clone()),
            ]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_tables_verbatim() {
        assert!(is_array_method("flatMap"));
        assert!(is_map_method("entries"));
        assert!(is_set_method("add"));
        assert!(!is_array_method("groupBy"));
    }

    #[test]
    fn spread_items_of_string_splits_chars() {
        assert_eq!(
            spread_items_of_json(&serde_json::Value::String("ab".to_string()))
                .unwrap()
                .len(),
            2
        );
    }
}
