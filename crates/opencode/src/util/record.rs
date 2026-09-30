// source: src/util/record.ts — `export * from "@opencode-ai/tui/util/record"`.
// PROVISIONAL pending @opencode-ai/tui (crates/tui): isRecord mirrored verbatim.

/// source: isRecord — plain-object check. Verbatim semantics for JSON values.
pub fn is_record(value: &serde_json::Value) -> bool {
    value.is_object()
}
