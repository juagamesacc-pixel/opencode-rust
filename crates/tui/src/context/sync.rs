// source: packages/tui/src/context/sync.tsx (673 lines, v1.18.30)
// 1:1 port — the SolidJS store becomes an explicit `SyncStore`; binary
// search, message keys, session hydration merge order, bootstrap sequencing
// (blocking vs non-blocking sets, partial→complete), and every event arm
// keep source order and semantics verbatim. Delivery arrives via
// `handle_event` (wired by the app loop).

#![allow(dead_code)]

use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::args::Args;
use super::event::EventMetadata;
use super::exit::Exit;
use super::kv::KvStore;
use super::permission::{PermissionMode, PermissionStore};
use super::project::ProjectStore;
use super::sdk::{data2, SdkClient};
use crate::runtime::relative_path;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Mirrors `search` — binary search over a sorted slice, returning
/// found + insertion index verbatim.
pub fn search_by<T>(items: &[T], target: &str, key: impl Fn(&T) -> &str) -> (bool, usize) {
    let mut left = 0usize;
    let mut right = items.len().saturating_sub(1);
    if items.is_empty() {
        return (false, 0);
    }
    while left <= right {
        let middle = (left + right) / 2;
        let value = key(&items[middle]);
        if value == target {
            return (true, middle);
        }
        if value < target {
            left = middle + 1;
        } else {
            if middle == 0 {
                break;
            }
            right = middle - 1;
        }
    }
    (false, left)
}

/// Owned-key variant for computed keys (mirrors `search` with `messageKey`).
pub fn search_by_key<T>(items: &[T], target: &str, key: impl Fn(&T) -> String) -> (bool, usize) {
    let mut left = 0usize;
    if items.is_empty() {
        return (false, 0);
    }
    let mut right = items.len().saturating_sub(1);
    while left <= right {
        let middle = (left + right) / 2;
        let value = key(&items[middle]);
        if value == target {
            return (true, middle);
        }
        if value.as_str() < target {
            left = middle + 1;
        } else {
            if middle == 0 {
                break;
            }
            right = middle - 1;
        }
    }
    (false, left)
}
pub fn message_key(message: &Value) -> String {
    let created = message
        .get("time")
        .and_then(|t| t.get("created"))
        .map(|c| c.to_string())
        .unwrap_or_default();
    let id = message.get("id").and_then(|i| i.as_str()).unwrap_or("");
    format!("{created}{id}")
}

/// Mirrors `compareMessage`.
pub fn compare_message(a: &Value, b: &Value) -> std::cmp::Ordering {
    let created = |m: &Value| {
        m.get("time")
            .and_then(|t| t.get("created"))
            .and_then(|c| c.as_i64())
            .unwrap_or(0)
    };
    created(a)
        .cmp(&created(b))
        .then_with(|| message_id(a).cmp(message_id(b)))
}

fn message_id(message: &Value) -> &str {
    message.get("id").and_then(|i| i.as_str()).unwrap_or("")
}

/// Mirrors the `status` union.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
    Loading,
    Partial,
    Complete,
}

/// Mirrors the Sync context value.
pub struct SyncStore {
    pub status: SyncStatus,
    pub provider: Vec<Value>,
    pub provider_default: HashMap<String, String>,
    pub provider_next: Value,
    pub console_state: Value,
    pub capabilities_background_subagents: bool,
    pub provider_auth: Value,
    pub agent: Vec<Value>,
    pub command: Vec<Value>,
    pub permission: HashMap<String, Vec<Value>>,
    pub question: HashMap<String, Vec<Value>>,
    pub config: Value,
    pub session: Vec<Value>,
    pub session_status: HashMap<String, Value>,
    pub session_diff: HashMap<String, Vec<Value>>,
    pub todo: HashMap<String, Vec<Value>>,
    pub message: HashMap<String, Vec<Value>>,
    pub part: HashMap<String, Vec<Value>>,
    pub lsp: Vec<Value>,
    pub mcp: HashMap<String, Value>,
    pub mcp_resource: HashMap<String, Value>,
    pub formatter: Vec<Value>,
    pub vcs: Option<Value>,
    pub skip_initial_loading: bool,
    full_synced_sessions: HashSet<String>,
    syncing_sessions: HashSet<String>,
    hydrating: HashMap<String, HydrationTracker>,
    client: Arc<dyn SdkClient>,
    kv_session_filter: bool,
    permission_mode: PermissionMode,
    args_continue: bool,
    exit: Exit,
}

#[derive(Default, Clone)]
struct HydrationTracker {
    messages: HashSet<String>,
    parts: HashSet<String>,
}

fn empty_console_state() -> Value {
    serde_json::json!({ "consoleManagedProviders": [], "switchableOrgCount": 0 })
}

impl SyncStore {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        client: Arc<dyn SdkClient>,
        args: &Args,
        kv: &KvStore,
        permission: &PermissionStore,
        startup_skip_loading: bool,
        exit: Exit,
    ) -> Self {
        Self {
            status: SyncStatus::Loading,
            provider: Vec::new(),
            provider_default: HashMap::new(),
            provider_next: serde_json::json!({ "all": [], "default": {}, "connected": [] }),
            console_state: empty_console_state(),
            capabilities_background_subagents: false,
            provider_auth: serde_json::json!({}),
            agent: Vec::new(),
            command: Vec::new(),
            permission: HashMap::new(),
            question: HashMap::new(),
            config: serde_json::json!({}),
            session: Vec::new(),
            session_status: HashMap::new(),
            session_diff: HashMap::new(),
            todo: HashMap::new(),
            message: HashMap::new(),
            part: HashMap::new(),
            lsp: Vec::new(),
            mcp: HashMap::new(),
            mcp_resource: HashMap::new(),
            formatter: Vec::new(),
            vcs: None,
            skip_initial_loading: startup_skip_loading,
            full_synced_sessions: HashSet::new(),
            syncing_sessions: HashSet::new(),
            hydrating: HashMap::new(),
            client,
            kv_session_filter: kv
                .get("session_directory_filter_enabled", Some(Value::Bool(true)))
                .as_bool()
                .unwrap_or(true),
            permission_mode: permission.mode,
            args_continue: args.continue_session,
            exit,
        }
    }

    /// Mirrors `ready`.
    pub fn ready(&self) -> bool {
        if self.skip_initial_loading {
            return true;
        }
        self.status != SyncStatus::Loading
    }

    /// Mirrors `sessionListQuery`.
    pub fn session_list_query(&self, project: &ProjectStore) -> Value {
        if !self.kv_session_filter {
            return serde_json::json!({ "scope": "project" });
        }
        let instance = project.instance_path();
        if instance.worktree.is_empty() || instance.directory.is_empty() {
            return serde_json::json!({ "scope": "project" });
        }
        let relative = relative_path(&instance.worktree, &instance.directory).replace('\\', "/");
        serde_json::json!({ "path": relative })
    }

    /// Mirrors `listSessions` (30-day window, id-sorted).
    pub async fn list_sessions(&self, project: &ProjectStore) -> Result<Vec<Value>, String> {
        let mut params = serde_json::json!({ "start": now_ms() - 30 * 24 * 60 * 60 * 1000 });
        if let Value::Object(query) = self.session_list_query(project) {
            if let Some(params_map) = params.as_object_mut() {
                params_map.extend(query);
            }
        }
        let response = self.client.call("session.list", params).await?;
        let mut sessions = response
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        sessions.sort_by(|a, b| message_id(a).cmp(message_id(b)));
        Ok(sessions)
    }

    /// Mirrors the event subscription switch.
    pub async fn handle_event(
        &mut self,
        payload: &Value,
        metadata: &EventMetadata,
        project: &mut ProjectStore,
    ) {
        let event_type = payload.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let props = payload.get("properties").unwrap_or(&Value::Null);
        let str_prop = |key: &str| {
            props
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };
        match event_type {
            "server.instance.disposed" => {
                let _ = self.bootstrap(project, true).await;
            }
            "permission.replied" => {
                let (session_id, request_id) = (str_prop("sessionID"), str_prop("requestID"));
                if let Some(requests) = self.permission.get_mut(&session_id) {
                    let (found, index) = search_by(requests, &request_id, |r| {
                        r.get("id").and_then(|v| v.as_str()).unwrap_or("")
                    });
                    if found && index < requests.len() {
                        requests.remove(index);
                    }
                }
            }
            "permission.asked" => {
                let request = props.clone();
                if self.permission_mode == PermissionMode::Auto {
                    let _ = self
                        .client
                        .call(
                            "permission.reply",
                            serde_json::json!({
                                "requestID": request.get("id"),
                                "reply": "once",
                                "directory": metadata.directory,
                                "workspace": metadata.workspace,
                            }),
                        )
                        .await;
                    return;
                }
                let session_id = str_prop("sessionID");
                let request_id = str_prop("id");
                match self.permission.get_mut(&session_id) {
                    None => {
                        self.permission.insert(session_id, vec![request]);
                    }
                    Some(requests) => {
                        let (found, index) = search_by(requests, &request_id, |r| {
                            r.get("id").and_then(|v| v.as_str()).unwrap_or("")
                        });
                        if found {
                            if index < requests.len() {
                                requests[index] = request;
                            }
                        } else if index <= requests.len() {
                            requests.insert(index, request);
                        } else {
                            requests.push(request);
                        }
                    }
                }
            }
            "question.replied" | "question.rejected" => {
                let (session_id, request_id) = (str_prop("sessionID"), str_prop("requestID"));
                if let Some(requests) = self.question.get_mut(&session_id) {
                    let (found, index) = search_by(requests, &request_id, |r| {
                        r.get("id").and_then(|v| v.as_str()).unwrap_or("")
                    });
                    if found && index < requests.len() {
                        requests.remove(index);
                    }
                }
            }
            "question.asked" => {
                let request = props.clone();
                let session_id = str_prop("sessionID");
                let request_id = str_prop("id");
                match self.question.get_mut(&session_id) {
                    None => {
                        self.question.insert(session_id, vec![request]);
                    }
                    Some(requests) => {
                        let (found, index) = search_by(requests, &request_id, |r| {
                            r.get("id").and_then(|v| v.as_str()).unwrap_or("")
                        });
                        if found {
                            if index < requests.len() {
                                requests[index] = request;
                            }
                        } else if index <= requests.len() {
                            requests.insert(index, request);
                        } else {
                            requests.push(request);
                        }
                    }
                }
            }
            "todo.updated" => {
                if let Some(todos) = props.get("todos").and_then(|t| t.as_array()).cloned() {
                    self.todo.insert(str_prop("sessionID"), todos);
                }
            }
            "session.diff" => {
                if let Some(diff) = props.get("diff").and_then(|d| d.as_array()).cloned() {
                    self.session_diff.insert(str_prop("sessionID"), diff);
                }
            }
            "session.deleted" => {
                let id = props
                    .get("info")
                    .and_then(|i| i.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let (found, index) = search_by(&self.session, id, |s| {
                    s.get("id").and_then(|v| v.as_str()).unwrap_or("")
                });
                if found && index < self.session.len() {
                    self.session.remove(index);
                }
            }
            "session.updated" => {
                let info = props.get("info").cloned().unwrap_or(Value::Null);
                let id = info
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let (found, index) = search_by(&self.session, &id, |s| {
                    s.get("id").and_then(|v| v.as_str()).unwrap_or("")
                });
                if found {
                    if index < self.session.len() {
                        self.session[index] = info;
                    }
                } else if index <= self.session.len() {
                    self.session.insert(index, info);
                } else {
                    self.session.push(info);
                }
            }
            "session.next.moved" => {
                let session_id = str_prop("sessionID");
                let (found, index) = search_by(&self.session, &session_id, |s| {
                    s.get("id").and_then(|v| v.as_str()).unwrap_or("")
                });
                if !found {
                    return;
                }
                if let Some(session) = self.session.get_mut(index) {
                    if let Some(map) = session.as_object_mut() {
                        map.insert(
                            "directory".to_string(),
                            props
                                .get("location")
                                .and_then(|l| l.get("directory"))
                                .cloned()
                                .unwrap_or(Value::Null),
                        );
                        map.insert(
                            "path".to_string(),
                            props.get("subdirectory").cloned().unwrap_or(Value::Null),
                        );
                        map.insert(
                            "workspaceID".to_string(),
                            props
                                .get("location")
                                .and_then(|l| l.get("workspaceID"))
                                .cloned()
                                .unwrap_or(Value::Null),
                        );
                        if let Some(time) = map.get_mut("time").and_then(|t| t.as_object_mut()) {
                            time.insert(
                                "updated".to_string(),
                                props.get("timestamp").cloned().unwrap_or(Value::Null),
                            );
                        }
                    }
                }
            }
            "session.status" => {
                self.session_status.insert(
                    str_prop("sessionID"),
                    props.get("status").cloned().unwrap_or(Value::Null),
                );
            }
            "message.updated" => {
                let info = props.get("info").cloned().unwrap_or(Value::Null);
                let session_id = info
                    .get("sessionID")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let info_id = message_id(&info).to_string();
                if let Some(tracker) = self.hydrating.get_mut(&session_id) {
                    tracker.messages.insert(info_id.clone());
                }
                let key = message_key(&info);
                match self.message.get_mut(&session_id) {
                    None => {
                        self.message.insert(session_id.clone(), vec![info]);
                    }
                    Some(messages) => {
                        let (found, index) = search_by_key(messages, &key, message_key);
                        if found {
                            if index < messages.len() {
                                messages[index] = info;
                            }
                        } else {
                            if index <= messages.len() {
                                messages.insert(index, info);
                            } else {
                                messages.push(info);
                            }
                            let updated_len =
                                self.message.get(&session_id).map(|m| m.len()).unwrap_or(0);
                            if updated_len > 100 {
                                if let Some(messages) = self.message.get_mut(&session_id) {
                                    if !messages.is_empty() {
                                        let oldest = messages.remove(0);
                                        let oldest_id = message_id(&oldest).to_string();
                                        self.part.remove(&oldest_id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            "message.removed" => {
                let (session_id, removed_id) = (str_prop("sessionID"), str_prop("messageID"));
                if let Some(tracker) = self.hydrating.get_mut(&session_id) {
                    tracker.messages.insert(removed_id.clone());
                }
                if let Some(messages) = self.message.get_mut(&session_id) {
                    if let Some(index) = messages.iter().position(|m| message_id(m) == removed_id) {
                        messages.remove(index);
                    }
                }
            }
            "message.part.updated" => {
                let part = props.get("part").cloned().unwrap_or(Value::Null);
                let part_message_id = part
                    .get("messageID")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let part_id = message_id(&part).to_string();
                if let Some(tracker) = self.hydrating.get_mut(
                    part.get("sessionID")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                        .as_str(),
                ) {
                    tracker.parts.insert(part_id.clone());
                }
                match self.part.get_mut(&part_message_id) {
                    None => {
                        self.part.insert(part_message_id, vec![part]);
                    }
                    Some(parts) => {
                        let (found, index) = search_by(parts, &part_id, |p| {
                            p.get("id").and_then(|v| v.as_str()).unwrap_or("")
                        });
                        if found {
                            if index < parts.len() {
                                parts[index] = part;
                            }
                        } else if index <= parts.len() {
                            parts.insert(index, part);
                        } else {
                            parts.push(part);
                        }
                    }
                }
            }
            "message.part.delta" => {
                let (message_id, part_id) = (str_prop("messageID"), str_prop("partID"));
                let delta = str_prop("delta");
                let Some(parts) = self.part.get_mut(&message_id) else {
                    return;
                };
                let (found, index) = search_by(parts, &part_id, |p| {
                    p.get("id").and_then(|v| v.as_str()).unwrap_or("")
                });
                if !found {
                    return;
                }
                if let Some(tracker) = self.hydrating.get_mut(&str_prop("sessionID")) {
                    tracker.parts.insert(part_id);
                }
                if let Some(part) = parts.get_mut(index) {
                    let field = props
                        .get("field")
                        .and_then(|f| f.as_str())
                        .unwrap_or("")
                        .to_string();
                    if let Some(map) = part.as_object_mut() {
                        let existing = map
                            .get(&field)
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        map.insert(field, Value::String(existing + &delta));
                    }
                }
            }
            "message.part.removed" => {
                let (message_id, part_id) = (str_prop("messageID"), str_prop("partID"));
                if let Some(tracker) = self.hydrating.get_mut(&str_prop("sessionID")) {
                    tracker.parts.insert(part_id.clone());
                }
                if let Some(parts) = self.part.get_mut(&message_id) {
                    let (found, index) = search_by(parts, &part_id, |p| {
                        p.get("id").and_then(|v| v.as_str()).unwrap_or("")
                    });
                    if found && index < parts.len() {
                        parts.remove(index);
                    }
                }
            }
            "lsp.updated" => {
                let workspace = project.workspace_current().map(str::to_string);
                let param = match workspace {
                    Some(w) => serde_json::json!({ "workspace": w }),
                    None => serde_json::json!({}),
                };
                if let Ok(response) = self.client.call("lsp.status", param).await {
                    self.lsp = response
                        .get("data")
                        .and_then(|d| d.as_array())
                        .cloned()
                        .unwrap_or_default();
                }
            }
            "vcs.branch.updated"
                if metadata.workspace.as_deref() == project.workspace_current() =>
            {
                self.vcs = Some(
                    serde_json::json!({ "branch": props.get("branch").cloned().unwrap_or(Value::Null) }),
                );
            }
            _ => {}
        }
    }

    /// Mirrors `bootstrap`.
    pub async fn bootstrap(
        &mut self,
        project: &mut ProjectStore,
        fatal: bool,
    ) -> Result<(), String> {
        let workspace = project.workspace_current().map(str::to_string);
        let ws_param = match &workspace {
            Some(w) => serde_json::json!({ "workspace": w }),
            None => serde_json::json!({}),
        };
        project.sync().await?;
        let sessions = if self.args_continue {
            Some(self.list_sessions(project).await?)
        } else {
            None
        };

        let providers = self.client.call("config.providers", ws_param.clone()).await;
        let provider_list = self.client.call("provider.list", ws_param.clone()).await;
        let capabilities = self
            .client
            .call("experimental.capabilities.get", ws_param.clone())
            .await
            .ok();
        let console_state = self
            .client
            .call("experimental.console.get", ws_param.clone())
            .await
            .ok();
        let agents = self.client.call("app.agents", ws_param.clone()).await;
        let config = self.client.call("config.get", ws_param.clone()).await;

        let failed =
            providers.is_err() || provider_list.is_err() || agents.is_err() || config.is_err();
        if failed {
            let message = providers
                .err()
                .or(provider_list.err())
                .or(agents.err())
                .or(config.err())
                .unwrap_or_default();
            eprintln!("tui bootstrap failed {message}");
            if fatal {
                self.exit.call(Some(message.clone()));
                return Err(message);
            }
            return Err(message);
        }
        if let Ok(response) = providers {
            if let Some(data) = response.get("data") {
                self.provider = data
                    .get("providers")
                    .and_then(|p| p.as_array())
                    .cloned()
                    .unwrap_or_default();
                self.provider_default = data
                    .get("default")
                    .and_then(|d| d.as_object())
                    .map(|map| {
                        map.iter()
                            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                            .collect()
                    })
                    .unwrap_or_default();
            }
        }
        if let Ok(response) = provider_list {
            if let Some(data) = response.get("data") {
                self.provider_next = data.clone();
            }
        }
        self.capabilities_background_subagents = capabilities
            .as_ref()
            .and_then(|c| c.get("data"))
            .and_then(|d| d.get("backgroundSubagents"))
            .and_then(|v| v.as_bool())
            == Some(true);
        if let Some(state) = console_state.as_ref().and_then(|c| c.get("data")) {
            self.console_state = state.clone();
        }
        if let Ok(response) = agents {
            self.agent = response
                .get("data")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
        }
        if let Ok(response) = config {
            if let Some(data) = response.get("data") {
                self.config = data.clone();
            }
        }
        if let Some(sessions) = sessions {
            self.session = sessions;
        }
        if self.status != SyncStatus::Complete {
            self.status = SyncStatus::Partial;
        }
        // Non-blocking set.
        if !self.args_continue {
            if let Ok(fresh) = self.list_sessions(project).await {
                self.session = fresh;
            }
        }
        if let Ok(state) = self
            .client
            .call("experimental.console.get", ws_param.clone())
            .await
        {
            if let Some(data) = state.get("data") {
                self.console_state = data.clone();
            }
        }
        let fetch = |method: &str| self.client.call(method, ws_param.clone());
        if let Ok(response) = fetch("command.list").await {
            self.command = response
                .get("data")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
        }
        if let Ok(response) = fetch("lsp.status").await {
            self.lsp = response
                .get("data")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
        }
        if let Ok(response) = fetch("mcp.status").await {
            self.mcp = response
                .get("data")
                .and_then(|d| d.as_object())
                .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default();
        }
        if let Ok(response) = fetch("experimental.resource.list").await {
            self.mcp_resource = response
                .get("data")
                .and_then(|d| d.as_object())
                .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default();
        }
        if let Ok(response) = fetch("formatter.status").await {
            self.formatter = response
                .get("data")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
        }
        if let Ok(response) = fetch("session.status").await {
            self.session_status = response
                .get("data")
                .and_then(|d| d.as_object())
                .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default();
        }
        if let Ok(response) = fetch("provider.auth").await {
            self.provider_auth = response.get("data").cloned().unwrap_or(Value::Null);
        }
        if let Ok(response) = fetch("vcs.get").await {
            self.vcs = response.get("data").cloned();
        }
        let _ = project.sync_workspace().await;
        self.status = SyncStatus::Complete;
        Ok(())
    }

    /// Mirrors `session.get`.
    pub fn session_get(&self, session_id: &str) -> Option<&Value> {
        let (found, index) = search_by(&self.session, session_id, |s| {
            s.get("id").and_then(|v| v.as_str()).unwrap_or("")
        });
        if found {
            self.session.get(index)
        } else {
            None
        }
    }

    /// Mirrors `session.status`.
    pub fn session_status_of(&self, session_id: &str) -> &str {
        let Some(session) = self.session_get(session_id) else {
            return "idle";
        };
        if session
            .get("time")
            .and_then(|t| t.get("compacting"))
            .and_then(|v| v.as_bool())
            == Some(true)
        {
            return "compacting";
        }
        let messages = self.message.get(session_id);
        let Some(last) = messages.and_then(|m| m.last()) else {
            return "idle";
        };
        if last.get("role").and_then(|r| r.as_str()) == Some("user") {
            return "working";
        }
        if last.get("time").and_then(|t| t.get("completed")).is_some() {
            "idle"
        } else {
            "working"
        }
    }

    /// Mirrors `session.refresh`.
    pub async fn session_refresh(&mut self, project: &ProjectStore) -> Result<(), String> {
        self.session = self.list_sessions(project).await?;
        Ok(())
    }

    /// Mirrors `session.sync` — hydration merge with touched-id tracking.
    pub async fn session_sync(&mut self, session_id: &str) -> Result<(), String> {
        if self.full_synced_sessions.contains(session_id) {
            return Ok(());
        }
        if self.syncing_sessions.contains(session_id) {
            return Ok(());
        }
        self.syncing_sessions.insert(session_id.to_string());
        self.hydrating
            .insert(session_id.to_string(), HydrationTracker::default());
        let result = self.session_sync_inner(session_id).await;
        self.syncing_sessions.remove(session_id);
        self.hydrating.remove(session_id);
        if result.is_ok() {
            self.full_synced_sessions.insert(session_id.to_string());
        }
        result
    }

    async fn session_sync_inner(&mut self, session_id: &str) -> Result<(), String> {
        let (session, messages, todo, diff) = tokio::join!(
            self.client.call(
                "session.get",
                serde_json::json!({ "sessionID": session_id })
            ),
            self.client.call(
                "session.messages",
                serde_json::json!({ "sessionID": session_id, "limit": 100 })
            ),
            self.client.call(
                "session.todo",
                serde_json::json!({ "sessionID": session_id })
            ),
            self.client.call(
                "session.diff",
                serde_json::json!({ "sessionID": session_id })
            ),
        );
        let session_data = data2(&session?).cloned().unwrap_or(Value::Null);
        let remote = messages?;
        let remote_list = remote
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        let todo_list = todo?.get("data").cloned();
        let diff_list = diff?.get("data").cloned();

        let tracker = self.hydrating.get(session_id).cloned().unwrap_or_default();
        let current_messages = self.message.get(session_id).cloned().unwrap_or_default();
        let mut infos: Vec<Value> = remote_list
            .iter()
            .filter_map(|message| {
                let info = message.get("info")?.clone();
                let id = message_id(&info).to_string();
                if !tracker.messages.contains(&id) {
                    return Some(info);
                }
                current_messages
                    .iter()
                    .find(|item| message_id(item) == id)
                    .cloned()
            })
            .collect();
        for message in &current_messages {
            let id = message_id(message).to_string();
            if tracker.messages.contains(&id) && !infos.iter().any(|item| message_id(item) == id) {
                infos.push(message.clone());
            }
        }
        infos.sort_by(compare_message);
        let removed: Vec<Value> = if infos.len() > 100 {
            infos[..infos.len() - 100].to_vec()
        } else {
            Vec::new()
        };
        let visible: Vec<Value> =
            infos[infos.len().saturating_sub(100.min(infos.len()))..].to_vec();
        let visible_ids: HashSet<String> =
            visible.iter().map(|m| message_id(m).to_string()).collect();

        let mut parts = self.part.clone();
        for message in &remote_list {
            let info_id = message
                .get("info")
                .and_then(|i| i.get("id"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if !visible_ids.contains(&info_id) {
                parts.remove(&info_id);
                continue;
            }
            let current_parts = parts.get(&info_id).cloned().unwrap_or_default();
            let mut merged: Vec<Value> = message
                .get("parts")
                .and_then(|p| p.as_array())
                .map(|list| {
                    list.iter()
                        .filter_map(|part| {
                            let id = message_id(part).to_string();
                            let current = current_parts.iter().find(|item| message_id(item) == id);
                            if tracker.parts.contains(&id) {
                                return current.cloned();
                            }
                            if let Some(current) = current {
                                let fresh_text =
                                    part.get("text").and_then(|t| t.as_str()).unwrap_or("");
                                let current_text =
                                    current.get("text").and_then(|t| t.as_str()).unwrap_or("");
                                let part_type =
                                    part.get("type").and_then(|t| t.as_str()).unwrap_or("");
                                let current_type =
                                    current.get("type").and_then(|t| t.as_str()).unwrap_or("");
                                if (part_type == "text" || part_type == "reasoning")
                                    && (current_type == "text" || current_type == "reasoning")
                                    && fresh_text.is_empty()
                                    && !current_text.is_empty()
                                {
                                    return Some(current.clone());
                                }
                            }
                            Some(part.clone())
                        })
                        .collect()
                })
                .unwrap_or_default();
            for part in &current_parts {
                let id = message_id(part).to_string();
                if tracker.parts.contains(&id) && !merged.iter().any(|item| message_id(item) == id)
                {
                    merged.push(part.clone());
                }
            }
            parts.insert(info_id, merged);
        }
        for message in &removed {
            parts.remove(message_id(message));
        }

        let (found, index) = search_by(&self.session, session_id, |s| {
            s.get("id").and_then(|v| v.as_str()).unwrap_or("")
        });
        if found {
            if index < self.session.len() {
                self.session[index] = session_data;
            }
        } else if index <= self.session.len() {
            self.session.insert(index, session_data);
        } else {
            self.session.push(session_data);
        }
        self.todo.insert(
            session_id.to_string(),
            todo_list
                .as_ref()
                .and_then(|t| t.as_array())
                .cloned()
                .unwrap_or_default(),
        );
        self.message.insert(session_id.to_string(), visible);
        self.part = parts;
        self.session_diff.insert(
            session_id.to_string(),
            diff_list
                .as_ref()
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default(),
        );
        Ok(())
    }
}
