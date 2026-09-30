//! Rust port of `src/main/debug.ts` (opencode v1.18.30).
//!
//! Ported exactly: the `focusableSelector` literal (byte-identical),
//! [`read_document_node_id`] and [`read_node_ids`] with their two
//! `Invalid DOM.… response` error strings, the dedupe/merge of the forced
//! node-id set, and the `["focus", "focus-visible"]` / `[]` pseudo-class
//! payloads.
//!
//! PROVISIONAL: `contents.debugger` (Electron CDP) has no in-workspace Rust
//! binding, so [`set_force_focus`] returns `unimplemented!()` while the
//! response readers and the selector stay 1:1.
//!
//! Original file: `packages/desktop/src/main/debug.ts`

/// Mirrors the `focusableSelector` template literal, byte-for-byte.
pub const FOCUSABLE_SELECTOR: &str = "\n  a[href],\n  button:not([disabled]),\n  input:not([disabled]),\n  select:not([disabled]),\n  textarea:not([disabled]),\n  summary,\n  [contenteditable=\"true\"],\n  [tabindex]:not([tabindex=\"-1\"])\n";

/// Mirrors the `["focus", "focus-visible"]` payload.
pub const FOCUS_PSEUDO_CLASSES: [&str; 2] = ["focus", "focus-visible"];

/// Mirrors the `[]` payload used when force-focus is turned off.
pub const NO_PSEUDO_CLASSES: [&str; 0] = [];

/// Mirrors the `Invalid DOM.getDocument response` string.
pub const INVALID_DOCUMENT_RESPONSE: &str = "Invalid DOM.getDocument response";

/// Mirrors the `Invalid DOM.querySelectorAll response` string.
pub const INVALID_QUERY_RESPONSE: &str = "Invalid DOM.querySelectorAll response";

/// Mirrors `function readDocumentNodeId(value)`.
pub fn read_document_node_id(value: &serde_json::Value) -> Result<i64, String> {
    let node_id = value
        .as_object()
        .and_then(|root| root.get("root"))
        .filter(|root| root.is_object())
        .and_then(|root| root.as_object())
        .and_then(|root| root.get("nodeId"))
        .and_then(|node_id| node_id.as_i64());
    node_id.ok_or_else(|| INVALID_DOCUMENT_RESPONSE.to_string())
}

/// Mirrors `function readNodeIds(value)`.
pub fn read_node_ids(value: &serde_json::Value) -> Result<Vec<i64>, String> {
    let Some(node_ids) = value.as_object().and_then(|record| record.get("nodeIds")) else {
        return Err(INVALID_QUERY_RESPONSE.to_string());
    };
    let Some(node_ids) = node_ids.as_array() else {
        return Err(INVALID_QUERY_RESPONSE.to_string());
    };
    let mut out = Vec::with_capacity(node_ids.len());
    for node_id in node_ids {
        match node_id.as_i64() {
            Some(node_id) => out.push(node_id),
            None => return Err(INVALID_QUERY_RESPONSE.to_string()),
        }
    }
    Ok(out)
}

/// Mirrors `forcedFocusNodes.set(contents, [...new Set([...prev, ...nodeIds])])`.
pub fn merge_node_ids(previous: &[i64], next: &[i64]) -> Vec<i64> {
    let mut merged: Vec<i64> = previous.to_vec();
    for node_id in next {
        if !merged.contains(node_id) {
            merged.push(*node_id);
        }
    }
    merged
}

/// Mirrors the `CSS.forcePseudoState` params for the force-focus-off pass.
pub fn force_pseudo_state_off(node_id: i64) -> serde_json::Value {
    serde_json::json!({ "nodeId": node_id, "forcedPseudoClasses": NO_PSEUDO_CLASSES })
}

/// Mirrors the `CSS.forcePseudoState` params for the force-focus-on pass.
pub fn force_pseudo_state_on(node_id: i64) -> serde_json::Value {
    serde_json::json!({ "nodeId": node_id, "forcedPseudoClasses": FOCUS_PSEUDO_CLASSES })
}

/// Mirrors the `DOM.getDocument` params.
pub fn get_document_params() -> serde_json::Value {
    serde_json::json!({ "depth": -1, "pierce": true })
}

/// Mirrors the `DOM.querySelectorAll` params.
pub fn query_selector_all_params(node_id: i64) -> serde_json::Value {
    serde_json::json!({ "nodeId": node_id, "selector": FOCUSABLE_SELECTOR })
}

/// Mirrors the CDP protocol version the debugger is attached with.
pub const DEBUGGER_PROTOCOL_VERSION: &str = "1.3";

/// Mirrors `export async function setForceFocus(contents, enabled)`.
///
/// PROVISIONAL(packages/desktop/src/main/debug.ts): needs
/// `contents.debugger` (Electron CDP).
pub fn set_force_focus(_contents: &(), _enabled: bool) -> ! {
    unimplemented!("setForceFocus needs the Electron CDP debugger binding")
}

#[cfg(test)]
mod tests {
    // The source ships no `debug.test.ts`; these cover the two response
    // readers the source throws on and the node-id merge.
    use super::*;

    #[test]
    fn reads_the_document_node_id() {
        assert_eq!(
            read_document_node_id(&serde_json::json!({ "root": { "nodeId": 7 } })).unwrap(),
            7
        );
    }

    #[test]
    fn rejects_a_malformed_document_response() {
        for value in [
            serde_json::json!(null),
            serde_json::json!({}),
            serde_json::json!({ "root": null }),
            serde_json::json!({ "root": "nope" }),
            serde_json::json!({ "root": {} }),
            serde_json::json!({ "root": { "nodeId": "7" } }),
        ] {
            assert_eq!(
                read_document_node_id(&value).unwrap_err(),
                INVALID_DOCUMENT_RESPONSE
            );
        }
    }

    #[test]
    fn reads_the_query_node_ids() {
        assert_eq!(
            read_node_ids(&serde_json::json!({ "nodeIds": [1, 2, 3] })).unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn rejects_a_malformed_query_response() {
        for value in [
            serde_json::json!(null),
            serde_json::json!({}),
            serde_json::json!({ "nodeIds": "1" }),
            serde_json::json!({ "nodeIds": [1, "2"] }),
        ] {
            assert_eq!(read_node_ids(&value).unwrap_err(), INVALID_QUERY_RESPONSE);
        }
    }

    #[test]
    fn merges_node_ids_without_duplicates() {
        assert_eq!(merge_node_ids(&[1, 2], &[2, 3]), vec![1, 2, 3]);
        assert_eq!(merge_node_ids(&[], &[4]), vec![4]);
    }

    #[test]
    fn the_selector_matches_the_source_literal() {
        assert!(FOCUSABLE_SELECTOR.starts_with('\n'));
        assert!(FOCUSABLE_SELECTOR.ends_with("[tabindex]:not([tabindex=\"-1\"])\n"));
        assert!(FOCUSABLE_SELECTOR.contains("  [contenteditable=\"true\"],\n"));
        assert_eq!(FOCUSABLE_SELECTOR.split(",\n").count(), 8);
    }
}
