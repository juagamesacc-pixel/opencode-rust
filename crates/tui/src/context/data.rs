// source: packages/tui/src/context/data.tsx (569 lines, v1.18.30)
// 1:1 port — the SolidJS store becomes an explicit `DataStore`; every
// `session.next.*` event arm keeps its mutation order verbatim. SDK shapes
// stay `serde_json::Value`; refresh calls use the `SdkClient` seam. Event
// delivery arrives via `handle_event` (wired by the app loop) instead of
// SolidJS subscriptions.

#![allow(dead_code)]

use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::Arc;

use super::event::EventMetadata;
use super::sdk::{data2, SdkClient};

/// Mirrors `LocationRef` (`workspaceID` optional).
#[derive(Debug, Clone, Default)]
pub struct LocationRef {
    pub directory: String,
    pub workspace_id: Option<String>,
}

/// Mirrors `locationKey` — `JSON.stringify([directory, workspaceID])`.
pub fn location_key(directory: &str, workspace_id: Option<&str>) -> String {
    match workspace_id {
        Some(id) => format!("[\"{}\",\"{}\"]", escape(directory), escape(id)),
        None => format!("[\"{}\",null]", escape(directory)),
    }
}

fn escape(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Mirrors `locationQuery`.
pub fn location_query(reference: Option<&LocationRef>) -> Value {
    match reference {
        Some(reference) => serde_json::json!({
            "directory": reference.directory,
            "workspace": reference.workspace_id,
        }),
        None => serde_json::json!({}),
    }
}

fn msg_id(item: &Value) -> &str {
    item.get("id").and_then(|v| v.as_str()).unwrap_or("")
}

fn msg_type(item: &Value) -> &str {
    item.get("type").and_then(|v| v.as_str()).unwrap_or("")
}

fn obj_mut(item: &mut Value) -> Option<&mut Map<String, Value>> {
    item.as_object_mut()
}

fn time_value(timestamp: &Value) -> Value {
    serde_json::json!({ "created": timestamp })
}

/// Mirrors `message.prepend` — dedupe by id, unshift otherwise.
pub fn prepend(messages: &mut Vec<Value>, item: Value) {
    let id = msg_id(&item).to_string();
    if messages.iter().any(|existing| msg_id(existing) == id) {
        return;
    }
    messages.insert(0, item);
}

/// Mirrors `message.activeAssistant`.
pub fn active_assistant(messages: &[Value]) -> Option<&Value> {
    messages.iter().find(|item| {
        msg_type(item) == "assistant" && item.get("time").and_then(|t| t.get("completed")).is_none()
    })
}

pub fn active_assistant_mut(messages: &mut [Value]) -> Option<&mut Value> {
    messages.iter_mut().find(|item| {
        msg_type(item) == "assistant" && item.get("time").and_then(|t| t.get("completed")).is_none()
    })
}

/// Mirrors `message.assistant`.
pub fn assistant<'a>(messages: &'a [Value], message_id: &str) -> Option<&'a Value> {
    messages
        .iter()
        .find(|item| msg_type(item) == "assistant" && msg_id(item) == message_id)
}

pub fn assistant_mut<'a>(messages: &'a mut [Value], message_id: &str) -> Option<&'a mut Value> {
    messages
        .iter_mut()
        .find(|item| msg_type(item) == "assistant" && msg_id(item) == message_id)
}

/// Mirrors `message.activeShell`.
pub fn active_shell_mut<'a>(messages: &'a mut [Value], call_id: &str) -> Option<&'a mut Value> {
    messages.iter_mut().find(|item| {
        msg_type(item) == "shell" && item.get("callID").and_then(|v| v.as_str()) == Some(call_id)
    })
}

fn assistant_content_mut<'a>(
    messages: &'a mut [Value],
    message_id: &str,
) -> Option<&'a mut Vec<Value>> {
    let item = assistant_mut(messages, message_id)?;
    item.get_mut("content")?.as_array_mut()
}

/// Mirrors `message.latestTool` (`findLast` by type + optional id).
pub fn latest_tool_mut<'a>(
    messages: &'a mut [Value],
    message_id: &str,
    call_id: Option<&str>,
) -> Option<&'a mut Value> {
    assistant_content_mut(messages, message_id)?
        .iter_mut()
        .rev()
        .find(|item| {
            msg_type(item) == "tool" && call_id.map(|id| msg_id(item) == id).unwrap_or(true)
        })
}

/// Mirrors `message.latestText`.
pub fn latest_text_mut<'a>(
    messages: &'a mut [Value],
    message_id: &str,
    text_id: &str,
) -> Option<&'a mut Value> {
    assistant_content_mut(messages, message_id)?
        .iter_mut()
        .rev()
        .find(|item| msg_type(item) == "text" && msg_id(item) == text_id)
}

/// Mirrors `message.latestReasoning`.
pub fn latest_reasoning_mut<'a>(
    messages: &'a mut [Value],
    message_id: &str,
    reasoning_id: &str,
) -> Option<&'a mut Value> {
    assistant_content_mut(messages, message_id)?
        .iter_mut()
        .rev()
        .find(|item| msg_type(item) == "reasoning" && msg_id(item) == reasoning_id)
}

/// Mirrors the Data context value.
pub struct DataStore {
    session_info: HashMap<String, Value>,
    session_message: HashMap<String, Vec<Value>>,
    session_permission: HashMap<String, Value>,
    session_question: HashMap<String, Value>,
    project_permission: HashMap<String, Value>,
    location: HashMap<String, Value>,
    default_location: LocationRef,
    client: Arc<dyn SdkClient>,
}

impl DataStore {
    pub fn new(client: Arc<dyn SdkClient>, directory: Option<String>) -> Self {
        Self {
            session_info: HashMap::new(),
            session_message: HashMap::new(),
            session_permission: HashMap::new(),
            session_question: HashMap::new(),
            project_permission: HashMap::new(),
            location: HashMap::new(),
            default_location: LocationRef {
                directory: directory.unwrap_or_default(),
                workspace_id: None,
            },
            client,
        }
    }

    /// Mirrors `message.update` — scoped mutation of one session's list.
    pub fn update_messages(&mut self, session_id: &str, update: impl FnOnce(&mut Vec<Value>)) {
        let messages = self
            .session_message
            .entry(session_id.to_string())
            .or_default();
        update(messages);
    }

    pub fn session_get(&self, session_id: &str) -> Option<&Value> {
        self.session_info.get(session_id)
    }

    pub async fn session_refresh(&mut self, session_id: &str) -> Result<(), String> {
        let response = self
            .client
            .call(
                "v2.session.get",
                serde_json::json!({ "sessionID": session_id }),
            )
            .await?;
        if let Some(data) = data2(&response).cloned() {
            self.session_info.insert(session_id.to_string(), data);
        }
        Ok(())
    }

    pub fn message_list(&self, session_id: &str) -> Option<&Vec<Value>> {
        self.session_message.get(session_id)
    }

    pub async fn message_refresh(&mut self, session_id: &str) -> Result<(), String> {
        let response = self
            .client
            .call(
                "v2.session.messages",
                serde_json::json!({ "sessionID": session_id }),
            )
            .await?;
        if let Some(data) = data2(&response).and_then(|d| d.as_array()).cloned() {
            self.session_message.insert(session_id.to_string(), data);
        }
        Ok(())
    }

    pub fn permission_list(&self, session_id: &str) -> Option<&Value> {
        self.session_permission.get(session_id)
    }

    pub async fn permission_refresh(&mut self, session_id: &str) -> Result<(), String> {
        let response = self
            .client
            .call(
                "v2.session.permission.list",
                serde_json::json!({ "sessionID": session_id }),
            )
            .await?;
        if let Some(data) = data2(&response).cloned() {
            self.session_permission.insert(session_id.to_string(), data);
        }
        Ok(())
    }

    pub fn question_list(&self, session_id: &str) -> Option<&Value> {
        self.session_question.get(session_id)
    }

    pub async fn question_refresh(&mut self, session_id: &str) -> Result<(), String> {
        let response = self
            .client
            .call(
                "v2.session.question.list",
                serde_json::json!({ "sessionID": session_id }),
            )
            .await?;
        if let Some(data) = data2(&response).cloned() {
            self.session_question.insert(session_id.to_string(), data);
        }
        Ok(())
    }

    pub fn project_permission_list(&self, project_id: &str) -> Option<&Value> {
        self.project_permission.get(project_id)
    }

    pub async fn project_permission_refresh(&mut self, project_id: &str) -> Result<(), String> {
        let response = self
            .client
            .call(
                "v2.permission.saved.list",
                serde_json::json!({ "projectID": project_id }),
            )
            .await?;
        if let Some(data) = data2(&response).cloned() {
            self.project_permission.insert(project_id.to_string(), data);
        }
        Ok(())
    }

    pub fn default_location(&self) -> &LocationRef {
        &self.default_location
    }

    fn location_entry_mut(&mut self, key: &str) -> &mut Value {
        self.location
            .entry(key.to_string())
            .or_insert_with(|| Value::Object(Map::new()))
    }

    /// Mirrors `location.refresh`.
    pub async fn location_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let response = self
            .client
            .call(
                "v2.location.get",
                serde_json::json!({ "location": location_query(reference) }),
            )
            .await?;
        let location = response.get("data").cloned().unwrap_or(Value::Null);
        let directory = location
            .get("directory")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let workspace_id = location
            .get("workspaceID")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let key = location_key(&directory, workspace_id.as_deref());
        self.location_entry_mut(&key);
        if reference.is_none() {
            self.default_location = LocationRef {
                directory,
                workspace_id,
            };
        }
        Ok(())
    }

    async fn location_list_refresh(
        &mut self,
        kind: &str,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let method = format!("v2.{kind}.list");
        let response = self
            .client
            .call(
                &method,
                serde_json::json!({ "location": location_query(reference) }),
            )
            .await?;
        let data = response.get("data").cloned().unwrap_or(Value::Null);
        let location = data.get("location").cloned().unwrap_or(Value::Null);
        let directory = location
            .get("directory")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let workspace_id = location
            .get("workspaceID")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let key = location_key(&directory, workspace_id.as_deref());
        if let (Some(entry), Some(items)) = (self.location.get_mut(&key), data.get("data")) {
            if let Some(map) = entry.as_object_mut() {
                map.insert(kind.to_string(), items.clone());
            }
        }
        Ok(())
    }

    pub fn location_agent_list(&self, reference: Option<&LocationRef>) -> Option<&Value> {
        let current = reference
            .cloned()
            .unwrap_or_else(|| self.default_location.clone());
        self.location
            .get(&location_key(
                &current.directory,
                current.workspace_id.as_deref(),
            ))?
            .get("agent")
    }

    pub async fn location_agent_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let owned = reference.cloned();
        self.location_list_refresh("agent", owned.as_ref()).await
    }

    pub fn location_command_list(&self, reference: Option<&LocationRef>) -> Option<&Value> {
        let current = reference
            .cloned()
            .unwrap_or_else(|| self.default_location.clone());
        self.location
            .get(&location_key(
                &current.directory,
                current.workspace_id.as_deref(),
            ))?
            .get("command")
    }

    pub async fn location_command_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let owned = reference.cloned();
        self.location_list_refresh("command", owned.as_ref()).await
    }

    pub fn location_integration_list(&self, reference: Option<&LocationRef>) -> Option<&Value> {
        let current = reference
            .cloned()
            .unwrap_or_else(|| self.default_location.clone());
        self.location
            .get(&location_key(
                &current.directory,
                current.workspace_id.as_deref(),
            ))?
            .get("integration")
    }

    pub async fn location_integration_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let owned = reference.cloned();
        self.location_list_refresh("integration", owned.as_ref())
            .await
    }

    pub fn location_model_list(&self, reference: Option<&LocationRef>) -> Option<&Value> {
        let current = reference
            .cloned()
            .unwrap_or_else(|| self.default_location.clone());
        self.location
            .get(&location_key(
                &current.directory,
                current.workspace_id.as_deref(),
            ))?
            .get("model")
    }

    pub async fn location_model_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let owned = reference.cloned();
        self.location_list_refresh("model", owned.as_ref()).await
    }

    pub fn location_provider_list(&self, reference: Option<&LocationRef>) -> Option<&Value> {
        let current = reference
            .cloned()
            .unwrap_or_else(|| self.default_location.clone());
        self.location
            .get(&location_key(
                &current.directory,
                current.workspace_id.as_deref(),
            ))?
            .get("provider")
    }

    pub async fn location_provider_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let owned = reference.cloned();
        self.location_list_refresh("provider", owned.as_ref()).await
    }

    pub fn location_reference_list(&self, reference: Option<&LocationRef>) -> Option<&Value> {
        let current = reference
            .cloned()
            .unwrap_or_else(|| self.default_location.clone());
        self.location
            .get(&location_key(
                &current.directory,
                current.workspace_id.as_deref(),
            ))?
            .get("reference")
    }

    pub async fn location_reference_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let owned = reference.cloned();
        self.location_list_refresh("reference", owned.as_ref())
            .await
    }

    pub fn location_skill_list(&self, reference: Option<&LocationRef>) -> Option<&Value> {
        let current = reference
            .cloned()
            .unwrap_or_else(|| self.default_location.clone());
        self.location
            .get(&location_key(
                &current.directory,
                current.workspace_id.as_deref(),
            ))?
            .get("skill")
    }

    pub async fn location_skill_refresh(
        &mut self,
        reference: Option<&LocationRef>,
    ) -> Result<(), String> {
        let owned = reference.cloned();
        self.location_list_refresh("skill", owned.as_ref()).await
    }

    /// Mirrors the mount-time `Promise.allSettled` refresh set.
    pub async fn refresh_all(&mut self) {
        let _ = self.location_refresh(None).await;
        let _ = self.location_agent_refresh(None).await;
        let _ = self.location_integration_refresh(None).await;
        let _ = self.location_model_refresh(None).await;
        let _ = self.location_provider_refresh(None).await;
        let _ = self.location_reference_refresh(None).await;
        let _ = self.location_command_refresh(None).await;
        let _ = self.location_skill_refresh(None).await;
    }

    /// Mirrors `handleEvent` — the full `V2Event` switch. `payload` carries
    /// the event properties, `location` the location envelope.
    pub async fn handle_event(&mut self, payload: &Value, metadata: &EventMetadata) {
        let event_type = payload.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let data = payload.get("properties").unwrap_or(&Value::Null).clone();
        let location = LocationRef {
            directory: metadata.directory.clone(),
            workspace_id: metadata.workspace.clone(),
        };
        let get_str = |value: &Value, key: &str| {
            value
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };
        match event_type {
            "catalog.updated" => {
                let _ = self.location_model_refresh(Some(&location)).await;
                let _ = self.location_provider_refresh(Some(&location)).await;
            }
            "session.next.agent.switched" => {
                let (session_id, message_id) =
                    (get_str(&data, "sessionID"), get_str(&data, "messageID"));
                self.update_messages(&session_id, |draft| {
                    prepend(
                        draft,
                        serde_json::json!({
                            "id": message_id,
                            "type": "agent-switched",
                            "agent": data.get("agent").cloned().unwrap_or(Value::Null),
                            "time": time_value(data.get("timestamp").unwrap_or(&Value::Null)),
                        }),
                    );
                });
            }
            "session.next.model.switched" => {
                let (session_id, message_id) =
                    (get_str(&data, "sessionID"), get_str(&data, "messageID"));
                self.update_messages(&session_id, |draft| {
                    prepend(
                        draft,
                        serde_json::json!({
                            "id": message_id,
                            "type": "model-switched",
                            "model": data.get("model").cloned().unwrap_or(Value::Null),
                            "time": time_value(data.get("timestamp").unwrap_or(&Value::Null)),
                        }),
                    );
                });
            }
            "session.next.prompted" => {
                let (session_id, message_id) =
                    (get_str(&data, "sessionID"), get_str(&data, "messageID"));
                self.update_messages(&session_id, |draft| {
                    prepend(
                        draft,
                        serde_json::json!({
                            "id": message_id,
                            "type": "user",
                            "text": data.get("prompt").and_then(|p| p.get("text")).cloned().unwrap_or(Value::Null),
                            "files": data.get("prompt").and_then(|p| p.get("files")).cloned().unwrap_or(Value::Null),
                            "agents": data.get("prompt").and_then(|p| p.get("agents")).cloned().unwrap_or(Value::Null),
                            "time": time_value(data.get("timestamp").unwrap_or(&Value::Null)),
                        }),
                    );
                });
            }
            "session.next.prompt.admitted" => {}
            "session.next.context.updated" => {
                let (session_id, message_id) =
                    (get_str(&data, "sessionID"), get_str(&data, "messageID"));
                self.update_messages(&session_id, |draft| {
                    prepend(
                        draft,
                        serde_json::json!({
                            "id": message_id,
                            "type": "system",
                            "text": data.get("text").cloned().unwrap_or(Value::Null),
                            "time": time_value(data.get("timestamp").unwrap_or(&Value::Null)),
                        }),
                    );
                });
            }
            "session.next.synthetic" => {
                let (session_id, message_id) =
                    (get_str(&data, "sessionID"), get_str(&data, "messageID"));
                self.update_messages(&session_id, |draft| {
                    prepend(
                        draft,
                        serde_json::json!({
                            "id": message_id,
                            "type": "synthetic",
                            "sessionID": data.get("sessionID").cloned().unwrap_or(Value::Null),
                            "text": data.get("text").cloned().unwrap_or(Value::Null),
                            "time": time_value(data.get("timestamp").unwrap_or(&Value::Null)),
                        }),
                    );
                });
            }
            "session.next.shell.started" => {
                let (session_id, message_id) =
                    (get_str(&data, "sessionID"), get_str(&data, "messageID"));
                self.update_messages(&session_id, |draft| {
                    prepend(
                        draft,
                        serde_json::json!({
                            "id": message_id,
                            "type": "shell",
                            "callID": data.get("callID").cloned().unwrap_or(Value::Null),
                            "command": data.get("command").cloned().unwrap_or(Value::Null),
                            "output": "",
                            "time": time_value(data.get("timestamp").unwrap_or(&Value::Null)),
                        }),
                    );
                });
            }
            "session.next.shell.ended" => {
                let (session_id, call_id) = (get_str(&data, "sessionID"), get_str(&data, "callID"));
                self.update_messages(&session_id, |draft| {
                    if let Some(item) = active_shell_mut(draft, &call_id) {
                        if let Some(map) = obj_mut(item) {
                            map.insert(
                                "output".to_string(),
                                data.get("output").cloned().unwrap_or(Value::Null),
                            );
                            if let Some(time) = map.get_mut("time").and_then(|t| t.as_object_mut())
                            {
                                time.insert(
                                    "completed".to_string(),
                                    data.get("timestamp").cloned().unwrap_or(Value::Null),
                                );
                            }
                        }
                    }
                });
            }
            "session.next.step.started" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let timestamp = data.get("timestamp").cloned().unwrap_or(Value::Null);
                self.update_messages(&session_id, |draft| {
                    if draft.iter().any(|message| msg_id(message) == assistant_id) {
                        return;
                    }
                    if let Some(current) = active_assistant_mut(draft) {
                        if let Some(time) = current.get_mut("time").and_then(|t| t.as_object_mut())
                        {
                            time.insert("completed".to_string(), timestamp.clone());
                        }
                    }
                    let snapshot = data
                        .get("snapshot")
                        .map(|s| serde_json::json!({ "start": s }));
                    let mut item = serde_json::json!({
                        "id": assistant_id,
                        "type": "assistant",
                        "agent": data.get("agent").cloned().unwrap_or(Value::Null),
                        "model": data.get("model").cloned().unwrap_or(Value::Null),
                        "content": [],
                        "time": time_value(&timestamp),
                    });
                    if let (Some(map), Some(snapshot)) = (obj_mut(&mut item), snapshot) {
                        map.insert("snapshot".to_string(), snapshot);
                    }
                    prepend(draft, item);
                });
            }
            "session.next.step.ended" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                self.update_messages(&session_id, |draft| {
                    let Some(current) = assistant_mut(draft, &assistant_id) else {
                        return;
                    };
                    let Some(map) = obj_mut(current) else { return };
                    if let Some(time) = map.get_mut("time").and_then(|t| t.as_object_mut()) {
                        time.insert(
                            "completed".to_string(),
                            data.get("timestamp").cloned().unwrap_or(Value::Null),
                        );
                    }
                    map.insert(
                        "finish".to_string(),
                        data.get("finish").cloned().unwrap_or(Value::Null),
                    );
                    map.insert(
                        "cost".to_string(),
                        data.get("cost").cloned().unwrap_or(Value::Null),
                    );
                    map.insert(
                        "tokens".to_string(),
                        data.get("tokens").cloned().unwrap_or(Value::Null),
                    );
                    if let Some(snapshot) = data.get("snapshot") {
                        let mut merged = map
                            .get("snapshot")
                            .cloned()
                            .unwrap_or(serde_json::json!({}));
                        if let Some(obj) = merged.as_object_mut() {
                            obj.insert("end".to_string(), snapshot.clone());
                        }
                        map.insert("snapshot".to_string(), merged);
                    }
                });
            }
            "session.next.step.failed" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                self.update_messages(&session_id, |draft| {
                    let Some(current) = assistant_mut(draft, &assistant_id) else {
                        return;
                    };
                    let Some(map) = obj_mut(current) else { return };
                    if let Some(time) = map.get_mut("time").and_then(|t| t.as_object_mut()) {
                        time.insert(
                            "completed".to_string(),
                            data.get("timestamp").cloned().unwrap_or(Value::Null),
                        );
                    }
                    map.insert("finish".to_string(), Value::String("error".to_string()));
                    map.insert(
                        "error".to_string(),
                        data.get("error").cloned().unwrap_or(Value::Null),
                    );
                });
            }
            "session.next.text.started" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let text_id = get_str(&data, "textID");
                self.update_messages(&session_id, |draft| {
                    if let Some(content) = assistant_content_mut(draft, &assistant_id) {
                        content
                            .push(serde_json::json!({ "type": "text", "id": text_id, "text": "" }));
                    }
                });
            }
            "session.next.text.delta" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let (text_id, delta) = (get_str(&data, "textID"), get_str(&data, "delta"));
                self.update_messages(&session_id, |draft| {
                    if let Some(item) = latest_text_mut(draft, &assistant_id, &text_id) {
                        let next = item
                            .get("text")
                            .and_then(|t| t.as_str())
                            .unwrap_or("")
                            .to_string()
                            + &delta;
                        if let Some(map) = obj_mut(item) {
                            map.insert("text".to_string(), Value::String(next));
                        }
                    }
                });
            }
            "session.next.text.ended" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let (text_id, text) = (
                    get_str(&data, "textID"),
                    data.get("text").cloned().unwrap_or(Value::Null),
                );
                self.update_messages(&session_id, |draft| {
                    if let Some(item) = latest_text_mut(draft, &assistant_id, &text_id) {
                        if let Some(map) = obj_mut(item) {
                            map.insert("text".to_string(), text);
                        }
                    }
                });
            }
            "session.next.tool.input.started" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let (call_id, name) = (get_str(&data, "callID"), get_str(&data, "name"));
                let timestamp = data.get("timestamp").cloned().unwrap_or(Value::Null);
                self.update_messages(&session_id, |draft| {
                    if let Some(content) = assistant_content_mut(draft, &assistant_id) {
                        content.push(serde_json::json!({
                            "type": "tool",
                            "id": call_id,
                            "name": name,
                            "time": time_value(&timestamp),
                            "state": { "status": "pending", "input": "" },
                        }));
                    }
                });
            }
            "session.next.tool.input.delta" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let (call_id, delta) = (get_str(&data, "callID"), get_str(&data, "delta"));
                self.update_messages(&session_id, |draft| {
                    if let Some(item) = latest_tool_mut(draft, &assistant_id, Some(&call_id)) {
                        let pending = item
                            .get("state")
                            .and_then(|s| s.get("status"))
                            .and_then(|s| s.as_str())
                            == Some("pending");
                        if pending {
                            let next = item
                                .get("state")
                                .and_then(|s| s.get("input"))
                                .and_then(|i| i.as_str())
                                .unwrap_or("")
                                .to_string()
                                + &delta;
                            if let Some(state) =
                                item.get_mut("state").and_then(|s| s.as_object_mut())
                            {
                                state.insert("input".to_string(), Value::String(next));
                            }
                        }
                    }
                });
            }
            "session.next.tool.input.ended" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let (call_id, text) = (
                    get_str(&data, "callID"),
                    data.get("text").cloned().unwrap_or(Value::Null),
                );
                self.update_messages(&session_id, |draft| {
                    if let Some(item) = latest_tool_mut(draft, &assistant_id, Some(&call_id)) {
                        let pending = item
                            .get("state")
                            .and_then(|s| s.get("status"))
                            .and_then(|s| s.as_str())
                            == Some("pending");
                        if pending {
                            if let Some(state) =
                                item.get_mut("state").and_then(|s| s.as_object_mut())
                            {
                                state.insert("input".to_string(), text);
                            }
                        }
                    }
                });
            }
            "session.next.tool.called" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let call_id = get_str(&data, "callID");
                self.update_messages(&session_id, |draft| {
                    let Some(item) = latest_tool_mut(draft, &assistant_id, Some(&call_id)) else {
                        return;
                    };
                    let Some(map) = obj_mut(item) else { return };
                    if let Some(time) = map.get_mut("time").and_then(|t| t.as_object_mut()) {
                        time.insert(
                            "ran".to_string(),
                            data.get("timestamp").cloned().unwrap_or(Value::Null),
                        );
                    }
                    map.insert(
                        "provider".to_string(),
                        data.get("provider").cloned().unwrap_or(Value::Null),
                    );
                    map.insert(
                        "state".to_string(),
                        serde_json::json!({
                            "status": "running",
                            "input": data.get("input").cloned().unwrap_or(Value::Null),
                            "structured": {},
                            "content": [],
                        }),
                    );
                });
            }
            "session.next.tool.progress" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let call_id = get_str(&data, "callID");
                self.update_messages(&session_id, |draft| {
                    let Some(item) = latest_tool_mut(draft, &assistant_id, Some(&call_id)) else {
                        return;
                    };
                    let running = item
                        .get("state")
                        .and_then(|s| s.get("status"))
                        .and_then(|s| s.as_str())
                        == Some("running");
                    if !running {
                        return;
                    }
                    if let Some(state) = item.get_mut("state").and_then(|s| s.as_object_mut()) {
                        state.insert(
                            "structured".to_string(),
                            data.get("structured").cloned().unwrap_or(Value::Null),
                        );
                        state.insert(
                            "content".to_string(),
                            data.get("content").cloned().unwrap_or(Value::Array(vec![])),
                        );
                    }
                });
            }
            "session.next.tool.success" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let call_id = get_str(&data, "callID");
                self.update_messages(&session_id, |draft| {
                    let Some(item) = latest_tool_mut(draft, &assistant_id, Some(&call_id)) else { return };
                    let running = item.get("state").and_then(|s| s.get("status")).and_then(|s| s.as_str()) == Some("running");
                    if !running {
                        return;
                    }
                    let input = item.get("state").and_then(|s| s.get("input")).cloned().unwrap_or(Value::Null);
                    let prev_provider = item.get("provider").cloned().unwrap_or(Value::Null);
                    let provider_data = data.get("provider").unwrap_or(&Value::Null);
                    let executed = provider_data.get("executed").and_then(|v| v.as_bool()).unwrap_or(false)
                        || prev_provider.get("executed").and_then(|v| v.as_bool()) == Some(true);
                    let Some(map) = obj_mut(item) else { return };
                    map.insert(
                        "state".to_string(),
                        serde_json::json!({
                            "status": "completed",
                            "input": input,
                            "structured": data.get("structured").cloned().unwrap_or(Value::Null),
                            "content": data.get("content").cloned().unwrap_or(Value::Array(vec![])),
                            "result": data.get("result").cloned().unwrap_or(Value::Null),
                        }),
                    );
                    map.insert(
                        "provider".to_string(),
                        serde_json::json!({
                            "executed": executed,
                            "metadata": prev_provider.get("metadata").cloned().unwrap_or(Value::Null),
                            "resultMetadata": provider_data.get("metadata").cloned().unwrap_or(Value::Null),
                        }),
                    );
                    if let Some(time) = map.get_mut("time").and_then(|t| t.as_object_mut()) {
                        time.insert("completed".to_string(), data.get("timestamp").cloned().unwrap_or(Value::Null));
                    }
                });
            }
            "session.next.tool.failed" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let call_id = get_str(&data, "callID");
                self.update_messages(&session_id, |draft| {
                    let Some(item) = latest_tool_mut(draft, &assistant_id, Some(&call_id)) else { return };
                    let status = item.get("state").and_then(|s| s.get("status")).and_then(|s| s.as_str()).unwrap_or("");
                    if status != "pending" && status != "running" {
                        return;
                    }
                    let state = item.get("state").cloned().unwrap_or(Value::Null);
                    let input = match state.get("input") {
                        Some(Value::String(_)) => serde_json::json!({}),
                        Some(input) => input.clone(),
                        None => serde_json::json!({}),
                    };
                    let running = status == "running";
                    let prev_provider = item.get("provider").cloned().unwrap_or(Value::Null);
                    let provider_data = data.get("provider").unwrap_or(&Value::Null);
                    let executed = provider_data.get("executed").and_then(|v| v.as_bool()).unwrap_or(false)
                        || prev_provider.get("executed").and_then(|v| v.as_bool()) == Some(true);
                    let Some(map) = obj_mut(item) else { return };
                    map.insert(
                        "state".to_string(),
                        serde_json::json!({
                            "status": "error",
                            "error": data.get("error").cloned().unwrap_or(Value::Null),
                            "input": input,
                            "structured": if running { state.get("structured").cloned().unwrap_or(Value::Null) } else { serde_json::json!({}) },
                            "content": if running { state.get("content").cloned().unwrap_or(Value::Array(vec![])) } else { serde_json::json!([]) },
                            "result": data.get("result").cloned().unwrap_or(Value::Null),
                        }),
                    );
                    map.insert(
                        "provider".to_string(),
                        serde_json::json!({
                            "executed": executed,
                            "metadata": prev_provider.get("metadata").cloned().unwrap_or(Value::Null),
                            "resultMetadata": provider_data.get("metadata").cloned().unwrap_or(Value::Null),
                        }),
                    );
                    if let Some(time) = map.get_mut("time").and_then(|t| t.as_object_mut()) {
                        time.insert("completed".to_string(), data.get("timestamp").cloned().unwrap_or(Value::Null));
                    }
                });
            }
            "session.next.reasoning.started" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let reasoning_id = get_str(&data, "reasoningID");
                let metadata = data.get("providerMetadata").cloned().unwrap_or(Value::Null);
                self.update_messages(&session_id, |draft| {
                    if let Some(content) = assistant_content_mut(draft, &assistant_id) {
                        content.push(serde_json::json!({
                            "type": "reasoning",
                            "id": reasoning_id,
                            "text": "",
                            "providerMetadata": metadata,
                        }));
                    }
                });
            }
            "session.next.reasoning.delta" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let (reasoning_id, delta) =
                    (get_str(&data, "reasoningID"), get_str(&data, "delta"));
                self.update_messages(&session_id, |draft| {
                    if let Some(item) = latest_reasoning_mut(draft, &assistant_id, &reasoning_id) {
                        let next = item
                            .get("text")
                            .and_then(|t| t.as_str())
                            .unwrap_or("")
                            .to_string()
                            + &delta;
                        if let Some(map) = obj_mut(item) {
                            map.insert("text".to_string(), Value::String(next));
                        }
                    }
                });
            }
            "session.next.reasoning.ended" => {
                let (session_id, assistant_id) = (
                    get_str(&data, "sessionID"),
                    get_str(&data, "assistantMessageID"),
                );
                let (reasoning_id, text) = (
                    get_str(&data, "reasoningID"),
                    data.get("text").cloned().unwrap_or(Value::Null),
                );
                let metadata = data.get("providerMetadata").cloned();
                self.update_messages(&session_id, |draft| {
                    if let Some(item) = latest_reasoning_mut(draft, &assistant_id, &reasoning_id) {
                        if let Some(map) = obj_mut(item) {
                            map.insert("text".to_string(), text);
                            if let Some(metadata) = metadata {
                                map.insert("providerMetadata".to_string(), metadata);
                            }
                        }
                    }
                });
            }
            "session.next.retried"
            | "session.next.compaction.started"
            | "session.next.compaction.delta" => {}
            "session.next.compaction.ended" => {
                let (session_id, message_id) =
                    (get_str(&data, "sessionID"), get_str(&data, "messageID"));
                self.update_messages(&session_id, |draft| {
                    prepend(
                        draft,
                        serde_json::json!({
                            "id": message_id,
                            "type": "compaction",
                            "reason": data.get("reason").cloned().unwrap_or(Value::Null),
                            "summary": data.get("text").cloned().unwrap_or(Value::Null),
                            "recent": data.get("recent").cloned().unwrap_or(Value::Null),
                            "time": time_value(data.get("timestamp").unwrap_or(&Value::Null)),
                        }),
                    );
                });
            }
            "reference.updated" => {
                let _ = self.location_reference_refresh(None).await;
            }
            "integration.updated" => {
                let _ = self.location_integration_refresh(Some(&location)).await;
                let _ = self.location_model_refresh(Some(&location)).await;
                let _ = self.location_provider_refresh(Some(&location)).await;
            }
            _ => {}
        }
    }
}
