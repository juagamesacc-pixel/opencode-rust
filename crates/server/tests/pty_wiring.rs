#![allow(clippy::all)]
// source: packages/server/src/handlers/pty.ts — wired to crates/core pty
// Now real: parse_cursor, not_found_message, ticket_scope mirror source verbatim and core types

use server::handlers_impl::pty::{
    not_found_message, parse_cursor, ticket_scope, PTY_CONNECT_TICKET_QUERY,
    PTY_CONNECT_TOKEN_HEADER, PTY_CONNECT_TOKEN_HEADER_VALUE,
};

#[test]
fn parse_cursor_cases() {
    assert_eq!(parse_cursor(Some("-1")), Some(-1));
    assert_eq!(parse_cursor(Some("0")), Some(0));
    assert_eq!(parse_cursor(Some("42")), Some(42));
    assert_eq!(parse_cursor(Some("-2")), None);
    assert_eq!(parse_cursor(Some("abc")), None);
    assert_eq!(parse_cursor(None), None);
}

#[test]
fn not_found_message_verbatim() {
    assert_eq!(not_found_message("abc123"), "PTY session not found: abc123");
}

#[test]
fn ticket_scope_verbatim() {
    let v = ticket_scope("/tmp/dir", Some("ws1"));
    assert_eq!(v["directory"], "/tmp/dir");
    assert_eq!(v["workspaceID"], "ws1");
    let v2 = ticket_scope("/tmp/dir", None);
    assert!(v2["workspaceID"].is_null());
}

#[test]
fn constants_verbatim() {
    assert_eq!(PTY_CONNECT_TOKEN_HEADER, "x-opencode-pty-connect-token");
    assert_eq!(PTY_CONNECT_TOKEN_HEADER_VALUE, "1");
    assert_eq!(PTY_CONNECT_TICKET_QUERY, "pty_ticket");
}

#[test]
fn wired_core_types_match() {
    // Verify wired core pty constants match real core API
    assert_eq!(core::pty::BUFFER_LIMIT, 2 * 1024 * 1024);
    assert_eq!(core::pty::EXITED_LIMIT, 25);
}
