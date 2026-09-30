//! Rust port of `packages/server/src/handlers/session.ts` (opencode v1.18.30).
//!
//! Source 305 lines: 18 ops under `server.session` (list/create/active/get/switchAgent/switchModel/prompt/compact/wait/revert.stage/clear/commit/context/history/events/interrupt/message).
//! Defaults: DefaultSessionsLimit=50, DefaultSessionHistoryLimit=50.
//! Error strings: "Invalid cursor", "Session not found: {id}", handler for Snaphsot.Error -> UnknownError with ref `err_<8>`.

pub const GROUP: &str = "server.session";
pub const OPERATIONS: &[&str] = &[
    "session.list",
    "session.create",
    "session.active",
    "session.get",
    "session.switchAgent",
    "session.switchModel",
    "session.prompt",
    "session.compact",
    "session.wait",
    "session.revert.stage",
    "session.revert.clear",
    "session.revert.commit",
    "session.context",
    "session.history",
    "session.events",
    "session.interrupt",
    "session.message",
];

pub const DEFAULT_SESSIONS_LIMIT: u64 = 50;
pub const DEFAULT_SESSION_HISTORY_LIMIT: u64 = 50;

pub const ERR_INVALID_CURSOR: &str = "Invalid cursor";
pub const ERR_SESSION_NOT_FOUND_PREFIX: &str = "Session not found: ";
pub const ERR_MESSAGE_NOT_FOUND_PREFIX: &str = "Message not found: ";
pub const ERR_PROMPT_CONFLICT_PREFIX: &str =
    "Prompt message ID conflicts with an existing durable record: ";
pub const ERR_UNKNOWN_MESSAGE: &str = "Unexpected server error. Check server logs for details.";
pub const ERR_SERVICE_UNAVAILABLE_TEMPLATE: &str = "Session {operation} is not available yet";

pub fn session_not_found_message(id: &str) -> String {
    format!("Session not found: {id}")
}

pub fn message_not_found_message(id: &str) -> String {
    format!("Message not found: {id}")
}

pub fn service_unavailable_message(operation: &str) -> String {
    format!("Session {operation} is not available yet")
}
