// source: packages/tui/src/context/editor.ts (408 lines, v1.18.30)
// 1:1 port — JSON-RPC editor bridge as an explicit state machine. Schemas
// become manual `Value` decoders; the WebSocket is a seam (`EditorSocket`
// + factory, wired by the app — no tungstenite offline); reconnect
// backoff (1s doubling to 10s) and the 1s Zed poll run off the render
// loop via `poll()`.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::editor::{editor_integration, EditorConnection, EditorIntegration};
use crate::editor_zed::{
    EditorPosition, EditorSelection, EditorSelectionRange, EditorSelectionSpan, ZedSelectionResult,
};
use crate::util::record::as_record;

/// Protocol version verbatim.
pub const MCP_PROTOCOL_VERSION: &str = "2025-11-25";
/// Reconnect backoff cap verbatim (10s).
pub const RECONNECT_MAX_MS: u64 = 10_000;
/// Zed poll interval verbatim (1s).
pub const ZED_POLL_MS: u64 = 1000;

/// Mirrors `EditorLabelState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorLabelState {
    Pending,
    Sent,
    None,
}

/// Mirrors the connection status union.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorStatus {
    Disabled,
    Connecting,
    Connected,
}

/// Mirrors `EditorMention`.
#[derive(Debug, Clone)]
pub struct EditorMention {
    pub file_path: String,
    pub line_start: i64,
    pub line_end: i64,
}

/// Mirrors the server info shape.
#[derive(Debug, Clone, Default)]
pub struct EditorServerInfo {
    pub protocol_version: Option<String>,
    pub server_name: Option<String>,
    pub server_version: Option<String>,
}

/// Outgoing JSON-RPC message.
#[derive(Debug, Clone)]
pub struct JsonRpcMessage {
    pub id: Option<Value>,
    pub method: Option<String>,
    pub params: Option<Value>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

/// Decode + validate a JSON-RPC message value.
pub fn decode_json_rpc(value: &Value) -> Option<JsonRpcMessage> {
    let map = as_record(value)?;
    Some(JsonRpcMessage {
        id: map.get("id").cloned(),
        method: map
            .get("method")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        params: map.get("params").cloned(),
        result: map.get("result").cloned(),
        error: map.get("error").cloned(),
    })
}

fn decode_position(value: &Value) -> Option<EditorPosition> {
    let map = as_record(value)?;
    Some(EditorPosition {
        line: map.get("line")?.as_i64()?,
        character: map.get("character")?.as_i64()?,
    })
}

fn decode_range(value: &Value) -> Option<EditorSelectionRange> {
    let map = as_record(value)?;
    Some(EditorSelectionRange {
        text: map.get("text")?.as_str()?.to_string(),
        selection: EditorSelectionSpan {
            start: decode_position(map.get("selection")?.get("start")?)?,
            end: decode_position(map.get("selection")?.get("end")?)?,
        },
    })
}

/// Decode the selection union, normalizing the single form into `ranges`
/// (mirrors the `decodeTo` transform; requires ≥1 range).
pub fn decode_editor_selection(value: &Value) -> Option<EditorSelection> {
    let map = as_record(value)?;
    let file_path = map.get("filePath")?.as_str()?.to_string();
    let source = map
        .get("source")
        .and_then(|v| v.as_str())
        .filter(|s| *s == "websocket" || *s == "zed")
        .map(str::to_string);
    if let Some(ranges) = map.get("ranges").and_then(|v| v.as_array()) {
        if ranges.is_empty() {
            return None;
        }
        let decoded: Option<Vec<EditorSelectionRange>> = ranges.iter().map(decode_range).collect();
        return Some(EditorSelection {
            file_path,
            source,
            ranges: decoded?,
        });
    }
    let single = decode_range(&{
        let mut single = serde_json::Map::new();
        single.insert("text".to_string(), map.get("text")?.clone());
        single.insert("selection".to_string(), map.get("selection")?.clone());
        Value::Object(single)
    })?;
    Some(EditorSelection {
        file_path,
        source,
        ranges: vec![single],
    })
}

/// Decode an `at_mentioned` payload.
pub fn decode_editor_mention(value: &Value) -> Option<EditorMention> {
    let map = as_record(value)?;
    Some(EditorMention {
        file_path: map.get("filePath")?.as_str()?.to_string(),
        line_start: map.get("lineStart")?.as_i64()?,
        line_end: map.get("lineEnd")?.as_i64()?,
    })
}

/// Decode the `initialize` result.
pub fn decode_server_info(value: &Value) -> Option<EditorServerInfo> {
    let map = as_record(value)?;
    let server = map.get("serverInfo");
    Some(EditorServerInfo {
        protocol_version: map
            .get("protocolVersion")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        server_name: server
            .and_then(|s| s.get("name"))
            .and_then(|v| v.as_str())
            .map(str::to_string),
        server_version: server
            .and_then(|s| s.get("version"))
            .and_then(|v| v.as_str())
            .map(str::to_string),
    })
}

/// Mirrors `editorSelectionKey` (`\0`-joined coordinates + text).
pub fn editor_selection_key(selection: Option<&EditorSelection>) -> String {
    let Some(selection) = selection else {
        return String::new();
    };
    let mut parts = vec![selection.file_path.clone()];
    for range in &selection.ranges {
        parts.push(range.selection.start.line.to_string());
        parts.push(range.selection.start.character.to_string());
        parts.push(range.selection.end.line.to_string());
        parts.push(range.selection.end.character.to_string());
        parts.push(range.text.clone());
    }
    parts.join("\0")
}

/// Parse an inbound socket text frame (string-only, verbatim).
pub fn parse_message(data: &str) -> Option<JsonRpcMessage> {
    serde_json::from_str::<Value>(data)
        .ok()
        .and_then(|value| decode_json_rpc(&value))
}

/// WebSocket seam (implemented by the app transport).
pub trait EditorSocket: Send {
    fn send_text(&mut self, text: String);
    fn close(&mut self);
    fn is_open(&self) -> bool;
}

/// Zed selection poll seam (default reads the Zed database).
pub trait ZedSelectionSource: Send {
    fn poll(&mut self, directory: &str) -> ZedSelectionResult;
}

pub struct DefaultZedSource;

impl ZedSelectionSource for DefaultZedSource {
    fn poll(&mut self, directory: &str) -> ZedSelectionResult {
        (editor_integration().selection)(directory)
    }
}

/// Editor context state.
pub struct EditorContext {
    pub status: EditorStatus,
    pub selection: Option<EditorSelection>,
    pub selection_sent: bool,
    pub server: Option<EditorServerInfo>,
    directory: String,
    cwd: String,
    port: Option<u16>,
    zed_terminal: bool,
    closed: bool,
    socket_open: bool,
    socket: Option<Box<dyn EditorSocket>>,
    reconnect_deadline: Option<Instant>,
    attempt: u32,
    request_id: u64,
    pending: HashMap<u64, String>,
    preserve_selection_on_reconnect: bool,
    last_zed_key: Option<String>,
    zed_dirty: bool,
    mention_listeners: Vec<Box<dyn FnMut(EditorMention) + Send>>,
    integration: EditorIntegration,
    has_integration_selection: bool,
}

impl EditorContext {
    pub fn new(cwd: &str, integration: Option<EditorIntegration>, has_selection_fn: bool) -> Self {
        let port = std::env::var("CLAUDE_CODE_SSE_PORT")
            .or_else(|_| std::env::var("OPENCODE_EDITOR_SSE_PORT"))
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|port| *port > 0 && *port <= 65535)
            .map(|port| port as u16);
        let zed_terminal = std::env::var("ZED_TERM")
            .map(|v| v == "true")
            .unwrap_or(false)
            || std::env::var("TERM_PROGRAM")
                .map(|v| v.to_lowercase() == "zed")
                .unwrap_or(false);
        Self {
            status: EditorStatus::Disabled,
            selection: None,
            selection_sent: false,
            server: None,
            directory: cwd.to_string(),
            cwd: cwd.to_string(),
            port,
            zed_terminal,
            closed: false,
            socket_open: false,
            socket: None,
            reconnect_deadline: None,
            attempt: 0,
            request_id: 0,
            pending: HashMap::new(),
            preserve_selection_on_reconnect: false,
            last_zed_key: None,
            zed_dirty: false,
            mention_listeners: Vec::new(),
            integration: integration.unwrap_or_else(editor_integration),
            has_integration_selection: has_selection_fn,
        }
    }

    /// Mount entry (mirrors `onMount(connect)`).
    pub fn connect(
        &mut self,
        factory: &mut dyn FnMut(&EditorConnection) -> Box<dyn EditorSocket>,
        zed: &mut dyn ZedSelectionSource,
    ) {
        self.connect_inner(factory, zed);
    }

    /// Unmount entry (mirrors `onCleanup`).
    pub fn shutdown(&mut self) {
        self.closed = true;
        self.reconnect_deadline = None;
        if let Some(socket) = self.socket.as_mut() {
            socket.close();
        }
        self.socket = None;
    }

    /// Render-loop driver (reconnect + Zed poll deadlines).
    pub fn poll(
        &mut self,
        factory: &mut dyn FnMut(&EditorConnection) -> Box<dyn EditorSocket>,
        zed: &mut dyn ZedSelectionSource,
    ) {
        if self.closed {
            return;
        }
        if self
            .reconnect_deadline
            .map(|deadline| Instant::now() >= deadline)
            .unwrap_or(false)
        {
            self.reconnect_deadline = None;
            self.connect_inner(factory, zed);
            return;
        }
        if self.socket.is_none() && self.zed_dirty {
            self.zed_dirty = false;
            self.poll_zed_once(zed);
            if !self.closed && self.socket.is_none() {
                self.reconnect_deadline = Some(Instant::now() + Duration::from_millis(ZED_POLL_MS));
            }
        }
    }

    fn resolve_connection(&self) -> Option<EditorConnection> {
        if let Some(port) = self.port {
            return Some(EditorConnection {
                url: format!("ws://127.0.0.1:{port}"),
                auth_token: None,
                source: format!("env:{port}"),
            });
        }
        (self.integration.connection)(&self.directory)
    }

    fn set_selection(&mut self, selection: Option<EditorSelection>) {
        let changed = editor_selection_key(selection.as_ref())
            != editor_selection_key(self.selection.as_ref());
        self.selection = selection;
        if changed {
            self.selection_sent = false;
        }
    }

    fn clear_for_reconnect(&mut self, reset_zed_key: bool) {
        if self.preserve_selection_on_reconnect {
            self.preserve_selection_on_reconnect = false;
            return;
        }
        if reset_zed_key {
            self.last_zed_key = None;
        }
        self.set_selection(None);
    }

    fn send(
        &mut self,
        id: Option<Value>,
        method: Option<&str>,
        params: Option<Value>,
        result: Option<Value>,
    ) {
        let Some(socket) = self.socket.as_mut() else {
            return;
        };
        if !socket.is_open() {
            return;
        }
        let mut payload = serde_json::json!({ "jsonrpc": "2.0" });
        if let Some(id) = id {
            payload["id"] = id;
        }
        if let Some(method) = method {
            payload["method"] = Value::String(method.to_string());
        }
        if let Some(params) = params {
            payload["params"] = params;
        }
        if let Some(result) = result {
            payload["result"] = result;
        }
        socket.send_text(payload.to_string());
    }

    fn request(&mut self, method: &str, params: Option<Value>) {
        self.request_id += 1;
        let id = self.request_id;
        self.pending.insert(id, method.to_string());
        self.send(Some(Value::Number(id.into())), Some(method), params, None);
    }

    fn schedule_reconnect(&mut self) {
        if self.closed {
            return;
        }
        self.attempt += 1;
        let delay =
            (1000u64.saturating_mul(2u64.saturating_pow(self.attempt - 1))).min(RECONNECT_MAX_MS);
        self.reconnect_deadline = Some(Instant::now() + Duration::from_millis(delay));
    }

    fn schedule_zed_poll(&mut self) {
        if self.closed {
            return;
        }
        self.reconnect_deadline = Some(Instant::now() + Duration::from_millis(ZED_POLL_MS));
    }

    fn connect_inner(
        &mut self,
        factory: &mut dyn FnMut(&EditorConnection) -> Box<dyn EditorSocket>,
        zed: &mut dyn ZedSelectionSource,
    ) {
        if self.closed {
            return;
        }
        let Some(connection) = self.resolve_connection() else {
            if !self.zed_terminal {
                self.status = EditorStatus::Disabled;
                self.schedule_reconnect();
                return;
            }
            if !self.has_integration_selection {
                self.status = EditorStatus::Disabled;
                self.schedule_reconnect();
                return;
            }
            self.poll_zed_once(zed);
            self.schedule_zed_poll();
            return;
        };
        self.status = EditorStatus::Connecting;
        // The transport reports `open` via `on_socket_open` (mirrors the
        // `open` listener: reset attempts, mark connected, initialize).
        self.socket = Some(factory(&connection));
        self.socket_open = false;
    }

    fn apply_zed_selection(&mut self, selection: Option<EditorSelection>) {
        let key = editor_selection_key(selection.as_ref());
        if Some(key.as_str()) != self.last_zed_key.as_deref() {
            self.last_zed_key = Some(key);
            let connected = selection.is_some();
            self.set_selection(selection);
            self.status = if connected {
                EditorStatus::Connected
            } else {
                EditorStatus::Disabled
            };
        }
    }

    fn poll_zed_once(&mut self, zed: &mut dyn ZedSelectionSource) {
        if self.closed || self.socket.is_some() {
            return;
        }
        match zed.poll(&self.directory) {
            // Unavailable keeps the last known selection (transient
            // polling failures must not clear it).
            ZedSelectionResult::Unavailable => {}
            ZedSelectionResult::Empty => self.apply_zed_selection(None),
            ZedSelectionResult::Selection { selection } => {
                self.apply_zed_selection(Some(selection))
            }
        }
    }

    /// Transport callback: socket opened (mirrors the `open` listener).
    pub fn on_socket_open(&mut self) {
        self.socket_open = true;
        self.attempt = 0;
        self.status = EditorStatus::Connected;
        self.request(
            "initialize",
            Some(serde_json::json!({
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": { "name": "opencode", "version": "0.0.0" },
            })),
        );
    }

    /// Transport callback: text frame (mirrors the `message` listener).
    pub fn on_socket_message(&mut self, data: &str) {
        let Some(message) = parse_message(data) else {
            return;
        };
        if message.method.as_deref() == Some("selection_changed") {
            if let Some(params) = message.params.as_ref().and_then(decode_editor_selection) {
                let mut selection = params;
                selection.source = Some("websocket".to_string());
                self.set_selection(Some(selection));
            }
            return;
        }
        if message.method.as_deref() == Some("at_mentioned") {
            if let Some(params) = message.params.as_ref().and_then(decode_editor_mention) {
                for listener in &mut self.mention_listeners {
                    listener(params.clone());
                }
            }
            return;
        }
        let Some(id) = message.id.as_ref().and_then(|v| v.as_u64()) else {
            return;
        };
        let Some(method) = self.pending.remove(&id) else {
            return;
        };
        if message.error.is_some() {
            return;
        }
        if method == "initialize" {
            if let Some(info) = message.result.as_ref().and_then(decode_server_info) {
                self.server = Some(info);
                self.send(None, Some("notifications/initialized"), None, None);
            }
        }
    }

    /// Transport callback: socket closed (mirrors the `close` listener).
    pub fn on_socket_close(&mut self) {
        self.socket = None;
        self.socket_open = false;
        self.pending.clear();
        if self.closed {
            return;
        }
        self.status = EditorStatus::Connecting;
        self.schedule_reconnect();
    }

    /// Directory switch (mirrors `reconnectWithDirectory`).
    pub fn reconnect(&mut self, next_directory: Option<&str>) {
        let resolved = next_directory.unwrap_or(&self.cwd).to_string();
        let same = self.directory == resolved;
        self.clear_for_reconnect(!same);
        if same {
            return;
        }
        self.directory = resolved;
        self.attempt = 0;
        self.pending.clear();
        self.reconnect_deadline = None;
        if let Some(socket) = self.socket.take() {
            let mut socket = socket;
            socket.close();
        }
        self.socket_open = false;
        self.status = EditorStatus::Disabled;
        self.server = None;
        self.zed_dirty = true;
    }

    pub fn enabled(&self) -> bool {
        self.resolve_connection().is_some() || (self.zed_terminal && self.has_integration_selection)
    }

    pub fn connected(&self) -> bool {
        self.status == EditorStatus::Connected
    }

    pub fn clear_selection(&mut self) {
        self.last_zed_key = None;
        self.set_selection(None);
    }

    pub fn preserve_selection_from_new_session(&mut self) {
        self.preserve_selection_on_reconnect = true;
    }

    pub fn mark_selection_sent(&mut self) {
        if self.selection.is_none() {
            return;
        }
        self.selection_sent = true;
    }

    pub fn label_state(&self) -> EditorLabelState {
        if self.selection.is_none() {
            return EditorLabelState::None;
        }
        if self.selection_sent {
            EditorLabelState::Sent
        } else {
            EditorLabelState::Pending
        }
    }

    pub fn on_mention(&mut self, listener: impl FnMut(EditorMention) + Send + 'static) {
        self.mention_listeners.push(Box::new(listener));
    }
}
