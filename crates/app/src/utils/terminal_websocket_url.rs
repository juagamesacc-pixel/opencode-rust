//! Rust port of `packages/app/src/utils/terminal-websocket-url.ts` (opencode v1.18.30).
//!
//! Source 35 lines: `terminalWebSocketURL` (verbatim V1/V2 paths, query
//! keys, `ws:`/`wss:` upgrade, ticket/auth-token rules).
//! Original file: `packages/app/src/utils/terminal-websocket-url.ts`

#![allow(dead_code)]

/// Mirrors the `terminalWebSocketURL` input shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalWebSocketInput {
    pub protocol: Option<String>,
    pub url: String,
    pub id: String,
    pub directory: String,
    pub cursor: i64,
    pub ticket: Option<String>,
    pub same_origin: bool,
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth_token: bool,
}

/// Mirrors `terminalWebSocketURL(input)` URL building.
pub fn terminal_websocket_url(input: &TerminalWebSocketInput) -> String {
    let is_v1 = input.protocol.as_deref() == Some("v1");
    let path = if is_v1 {
        format!("/pty/{}/connect", input.id)
    } else {
        format!("/api/pty/{}/connect", input.id)
    };
    let base = input.url.trim_end_matches('/');
    let scheme = if base.starts_with("https:") {
        "wss:"
    } else {
        "ws:"
    };
    let rest = base.split_once(':').map(|(_, after)| after).unwrap_or(base);
    let url = format!("{scheme}{rest}{path}");
    let mut params: Vec<String> = vec![];
    if is_v1 {
        params.push(format!("directory={}", input.directory));
    } else {
        params.push(format!("location[directory]={}", input.directory));
    }
    params.push(format!("cursor={}", input.cursor));
    if let Some(ticket) = &input.ticket {
        params.push(format!("ticket={ticket}"));
        return format!("{url}?{}", params.join("&"));
    }
    if is_v1 && input.password.is_some() && (!input.same_origin || input.auth_token) {
        params.push("auth_token=<credentials>".to_string());
    }
    format!("{url}?{}", params.join("&"))
}
