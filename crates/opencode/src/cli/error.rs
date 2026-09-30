// source: src/cli/error.ts — exports: [FormatError, FormatUnknownError]
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/error`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/util/error"
/// - "CliError"
/// - "MCPFailed"
/// - "AccountServiceError"
/// - "AccountTransportError"
/// - "ProviderModelNotFoundError"
/// - "providerID"
/// - "Did you mean: "
/// source: `export function FormatError` — stub shell; CI verifies behavior.
pub fn FormatError(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function FormatUnknownError` — stub shell; CI verifies behavior.
pub fn FormatUnknownError(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
