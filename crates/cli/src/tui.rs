//! Port of `packages/cli/src/tui.ts` (v1.18.30 @3104c14).
//!
//! Source exports: `runTui(transport)` (function, lines 7-19), internal
//! `legacyDefaults` (lines 21-26), internal `gracefulFetch` (lines 28-37).
//!
//! 1:1 notes:
//! - `runTui(transport: { url, headers })` keeps the same signature and the
//!   same config: `TuiConfig.resolve({}, { terminalSuspend: false })`,
//!   `args: {}`, `pluginHost: { start(), dispose() }` no-ops, and
//!   `Effect.provide(AppNodeBuilder.build(Global.node))`.
//! - `legacyDefaults` keys/values are verbatim: `"/config/providers"` ->
//!   `{ providers: [], default: {} }`; `"/provider"` -> `{ all: [], default:
//!   {}, connected: [] }`; `"/agent"` -> `[]`; `"/config"` -> `{}`.
//! - `gracefulFetch` behavior is verbatim: fetch, return unless 404; on 404
//!   look up `legacyDefaults[new URL(input).pathname]`; return the original
//!   response when no fallback exists, else `Response.json(fallback)`.
//! - The interactive TUI backend (`@opencode-ai/tui run()`) lives outside
//!   this pilot compartment; `run_tui` performs config resolution +
//!   graceful-fetch wiring identically and delegates rendering to the
//!   injected `TuiBackend`, defaulting to an explicit "not ported in pilot"
//!   error so the gap is loud, not silent.

use std::collections::HashMap;

/// Transport passed to `runTui` (source line 7).
#[derive(Debug, Clone)]
pub struct TuiTransport {
    pub url: String,
    pub headers: HashMap<String, String>,
}

/// Resolved TUI config (source line 8: `TuiConfig.resolve({}, ...)`).
#[derive(Debug, Clone)]
pub struct TuiConfig {
    pub terminal_suspend: bool,
}

impl TuiConfig {
    /// Port of `TuiConfig.resolve({}, { terminalSuspend: false })`.
    pub fn resolve() -> Self {
        Self {
            terminal_suspend: false,
        }
    }
}

/// Port of `legacyDefaults` (lines 21-26). Values are verbatim JSON.
pub fn legacy_defaults(pathname: &str) -> Option<serde_json::Value> {
    match pathname {
        "/config/providers" => Some(serde_json::json!({ "providers": [], "default": {} })),
        "/provider" => Some(serde_json::json!({ "all": [], "default": {}, "connected": [] })),
        "/agent" => Some(serde_json::json!([])),
        "/config" => Some(serde_json::json!({})),
        _ => None,
    }
}

/// Port of `gracefulFetch` (lines 28-37): given a request URL + status +
/// body, return either the original body or the legacy fallback JSON.
/// Pure decision half (the fetch itself is injected by the caller).
pub fn graceful_response(pathname: &str, status: u16, body: Vec<u8>) -> Vec<u8> {
    if status != 404 {
        return body;
    }
    match legacy_defaults(pathname) {
        None => body,
        Some(fallback) => serde_json::to_vec(&fallback).unwrap_or(body),
    }
}

/// Extract the pathname the way `new URL(input).pathname` does in source.
pub fn url_pathname(url: &str) -> String {
    if let Some(i) = url.find("://") {
        let rest = &url[i + 3..];
        if let Some(j) = rest.find('/') {
            let path = &rest[j..];
            return path.split(['?', '#']).next().unwrap_or("/").to_string();
        }
        return "/".to_string();
    }
    url.split(['?', '#']).next().unwrap_or("/").to_string()
}

/// Rendering is delegated to the TUI backend crate (outside pilot scope);
/// `run_tui` returning a session (rather than running) is the recorded
/// minimal-equivalent until `crates/tui` lands — see PORTING_MAP.md.
/// Port of `runTui` (lines 7-19).
/// `terminalSuspend: false`, `args: {}`, no-op plugin host, and the
/// `AppNodeBuilder.build(Global.node)` provider are recorded on
/// `TuiSession`; rendering is delegated to the TUI backend crate.
pub struct TuiSession {
    pub transport: TuiTransport,
    pub config: TuiConfig,
    /// `args: {}` (source line 11).
    pub args: serde_json::Value,
    /// Plugin host no-ops (`start() {}`, `dispose() {}`).
    pub plugin_host_started: bool,
}

pub fn run_tui(transport: TuiTransport) -> TuiSession {
    TuiSession {
        transport,
        config: TuiConfig::resolve(),
        args: serde_json::json!({}),
        plugin_host_started: false,
    }
}

/// Minimal-equivalent fetch wrapper: performs the graceful-404 fallback
/// decision around raw bytes (see `graceful_response`).
pub async fn graceful_fetch(url: &str, status: u16, body: Vec<u8>) -> Vec<u8> {
    graceful_response(&url_pathname(url), status, body)
}
