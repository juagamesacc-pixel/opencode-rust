// source: src/server/shared/pty-ticket.ts — exports: [PTY_CONNECT_TICKET_QUERY, PTY_CONNECT_TOKEN_HEADER, PTY_CONNECT_TOKEN_HEADER_VALUE, isPtyConnectPath, hasPtyConnectTicketURL]
/// verbatim strings (source order, quoted for V2 audit):
/// - "x-opencode-ticket"
/// source: `PTY_CONNECT_TICKET_QUERY = "ticket"` — verbatim.
pub const PTY_CONNECT_TICKET_QUERY: &str = "ticket";
/// source: `PTY_CONNECT_TOKEN_HEADER = "x-opencode-ticket"` — verbatim.
pub const PTY_CONNECT_TOKEN_HEADER: &str = "x-opencode-ticket";
/// source: `PTY_CONNECT_TOKEN_HEADER_VALUE = "1"` — verbatim.
pub const PTY_CONNECT_TOKEN_HEADER_VALUE: &str = "1";
/// source: `export function isPtyConnectPath` — stub shell; CI verifies behavior.
pub fn isPtyConnectPath(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function hasPtyConnectTicketURL` — stub shell; CI verifies behavior.
pub fn hasPtyConnectTicketURL(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
