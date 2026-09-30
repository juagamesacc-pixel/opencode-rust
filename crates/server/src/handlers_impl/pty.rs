//! Rust port of `packages/server/src/handlers/pty.ts` (opencode v1.18.30).
//!
//! Source 199 lines: `PtyHandler` with 7 ops: list/create/get/update/remove/connectToken/connect(raw).
//! PTY connect token header `PTY_CONNECT_TOKEN_HEADER` / value, ticketScope, single-writer Queue,
//! PtyProtocol chunks/meta, cursor parsing, ticket consume, close codes 4404.
//!
//! WIRED: `Pty.Service` types (Info/CreateInput/UpdateInput/NotFoundError/ExitedError/BUFFER_LIMIT) now from `core::pty`
//! (verified against crates/core/src/pty.rs). Remaining PROVISIONAL: `PtyTicket.Service` and `PtyProtocol`
//! (ticketScope/Queue chunks/meta) — no core API yet for ticket/protocol, keep marked with reason.

pub const GROUP: &str = "server.pty";
pub const OPERATIONS: &[&str] = &[
    "pty.list",
    "pty.create",
    "pty.get",
    "pty.update",
    "pty.remove",
    "pty.connectToken",
    "pty.connect",
];

pub const PTY_CONNECT_TOKEN_HEADER: &str = "x-opencode-pty-connect-token";
pub const PTY_CONNECT_TOKEN_HEADER_VALUE: &str = "1";
pub const PTY_CONNECT_TICKET_QUERY: &str = "pty_ticket";

pub const PTY_NOT_FOUND_MESSAGE: &str = "PTY session not found: ";
pub const CLOSE_CODE_NOT_FOUND: u16 = 4404;
pub const CLOSE_MESSAGE_NOT_FOUND: &str = "session not found";
pub const CLOSE_MESSAGE_EXITED: &str = "session exited";
pub const CURSOR_DEFAULT: i64 = -1;

// Re-export wired core types for handler use — verified signatures from core::pty
pub use core::pty::{CreateInput, Info, UpdateInput};

pub fn not_found_message(pty_id: &str) -> String {
    format!("PTY session not found: {pty_id}")
}

pub fn parse_cursor(value: Option<&str>) -> Option<i64> {
    let s = value?;
    let n: i64 = s.parse().ok()?;
    if n >= -1 {
        Some(n)
    } else {
        None
    }
}

/// Pure branch: ticket scope is `{ directory, workspaceID }` from Location — mirrors source ticketScope gen.
pub fn ticket_scope(directory: &str, workspace_id: Option<&str>) -> serde_json::Value {
    serde_json::json!({ "directory": directory, "workspaceID": workspace_id })
}
