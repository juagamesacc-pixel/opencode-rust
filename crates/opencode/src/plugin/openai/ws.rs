// source: src/plugin/openai/ws.ts — exports: PROTOCOL_HEADER,
// MESSAGE_TOO_BIG_CLOSE_CODE, ConnectResponsesWebSocketOptions,
// StreamResponsesWebSocketOptions, WrappedError, toWebSocketUrl,
// normalizeHeaders, isAbortError, connectResponsesWebSocket,
// streamResponsesWebSocket, OpenAIWebSocket
// PROVISIONAL: ws runtime as descriptors; header/code/URL/header rules verbatim.

/// source: PROTOCOL_HEADER — verbatim.
pub const PROTOCOL_HEADER: &str = "responses_websockets=2026-02-06";
/// source: MESSAGE_TOO_BIG_CLOSE_CODE = 1009 — verbatim.
pub const MESSAGE_TOO_BIG_CLOSE_CODE: u16 = 1009;

/// source: toWebSocketUrl() — ^http → ws. Verbatim.
pub fn to_websocket_url(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("http") {
        return format!("ws{}", rest);
    }
    url.to_string()
}

/// source: normalizeHeaders() — lowercase keys, skip null. Verbatim rule.
pub fn normalize_header_key(key: &str) -> String {
    key.to_lowercase()
}
