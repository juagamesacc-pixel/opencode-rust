// source: packages/tui/src/context/local.tsx (542 lines, v1.18.30)
// 1:1 port — agent/model/session/mcp state as explicit structs; memos
// become methods over the sync store; model.json/session.json persistence
// keeps the ready/pending sequencing verbatim; toasts flow through a
// minimal `LocalToast` seam.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use super::route::{Route, RouteStore};
use super::sdk::SdkClient;
use super::sync::SyncStore;
use crate::theme::{Rgba, Theme};
use crate::util::selection::ToastVariant;

/// Mirrors the `{ providerID, modelID }` pair.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModelId {
    pub provider_id: String,
    pub model_id: String,
}

/// Mirrors `parseModel`.
pub fn parse_model(model: &str) -> ModelId {
    match model.split_once('/') {
        Some((provider_id, model_id)) => ModelId {
            provider_id: provider_id.to_string(),
            model_id: model_id.to_string(),
        },
        None => ModelId {
            provider_id: model.to_string(),
            model_id: String::new(),
        },
    }
}

/// Mirrors `recentModels` — current first, deduped, capped at 10.
pub fn recent_models(current: ModelId, recent: &[ModelId]) -> Vec<ModelId> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for item in std::iter::once(&current).chain(recent.iter()) {
        let key = format!("{}/{}", item.provider_id, item.model_id);
        if seen.insert(key) {
            out.push(item.clone());
        }
        if out.len() >= 10 {
            break;
        }
    }
    out
}

/// Minimal toast surface the local context needs.
pub trait LocalToast {
    fn show(&mut self, variant: ToastVariant, message: &str, duration_ms: u64);
}

/// Toast durations verbatim.
pub const TOAST_DURATION_MS: u64 = 3000;

fn agent_value<'a>(agents: &'a [Value], name: &str) -> Option<&'a Value> {
    agents
        .iter()
        .find(|a| a.get("name").and_then(|v| v.as_str()) == Some(name))
}

fn theme_named_color(theme: &Theme, name: &str) -> Rgba {
    match name {
        "primary" => theme.primary,
        "secondary" => theme.secondary,
        "accent" => theme.accent,
        "success" => theme.success,
        "warning" => theme.warning,
        "error" => theme.error,
        "info" => theme.info,
        _ => theme.secondary,
    }
}

/// Agent selection state.
#[derive(Debug, Default)]
pub struct AgentState {
    pub current: Option<String>,
}

/// Model selection + persistence state.
#[derive(Debug, Default)]
pub struct ModelState {
    pub ready: bool,
    pub per_agent: HashMap<String, ModelId>,
    pub recent: Vec<ModelId>,
    pub favorite: Vec<ModelId>,
    pub variant: HashMap<String, String>,
}

/// Session pin state.
#[derive(Debug, Default)]
pub struct SessionPinState {
    pub ready: bool,
    pub pinned: Vec<String>,
}

/// Mirrors the Local context value.
pub struct LocalContext {
    pub agent: AgentState,
    pub model: ModelState,
    model_file: PathBuf,
    model_pending: bool,
    pub session: SessionPinState,
    session_file: PathBuf,
    session_pending: bool,
}

impl LocalContext {
    pub fn new(state_dir: &str) -> Self {
        Self {
            agent: AgentState::default(),
            model: ModelState::default(),
            model_file: PathBuf::from(state_dir).join("model.json"),
            model_pending: false,
            session: SessionPinState::default(),
            session_file: PathBuf::from(state_dir).join("session.json"),
            session_pending: false,
        }
    }

    /// Load model.json (mirrors the read effect; pending saves flush after).
    pub async fn load_model(&mut self) {
        if let Ok(text) = tokio::fs::read_to_string(&self.model_file).await {
            if let Ok(value) = serde_json::from_str::<Value>(&text) {
                if let Some(recent) = value.get("recent").and_then(|v| v.as_array()) {
                    self.model.recent = recent
                        .iter()
                        .filter_map(|item| {
                            Some(ModelId {
                                provider_id: item.get("providerID")?.as_str()?.to_string(),
                                model_id: item.get("modelID")?.as_str()?.to_string(),
                            })
                        })
                        .collect();
                }
                if let Some(favorite) = value.get("favorite").and_then(|v| v.as_array()) {
                    self.model.favorite = favorite
                        .iter()
                        .filter_map(|item| {
                            Some(ModelId {
                                provider_id: item.get("providerID")?.as_str()?.to_string(),
                                model_id: item.get("modelID")?.as_str()?.to_string(),
                            })
                        })
                        .collect();
                }
                if let Some(variant) = value.get("variant").and_then(|v| v.as_object()) {
                    self.model.variant = variant
                        .iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect();
                }
            }
        }
        self.model.ready = true;
        if self.model_pending {
            self.save_model();
        }
    }

    /// Load session.json (pinned filter keeps strings only).
    pub async fn load_session(&mut self) {
        if let Ok(text) = tokio::fs::read_to_string(&self.session_file).await {
            if let Ok(value) = serde_json::from_str::<Value>(&text) {
                if let Some(pinned) = value.get("pinned").and_then(|v| v.as_array()) {
                    self.session.pinned = pinned
                        .iter()
                        .filter_map(|item| item.as_str().map(str::to_string))
                        .collect();
                }
            }
        }
        self.session.ready = true;
        if self.session_pending {
            self.save_session();
        }
    }

    fn save_model(&mut self) {
        if !self.model.ready {
            self.model_pending = true;
            return;
        }
        self.model_pending = false;
        let file = self.model_file.clone();
        let payload = serde_json::json!({
            "recent": self.model.recent.iter().map(|m| serde_json::json!({ "providerID": m.provider_id, "modelID": m.model_id })).collect::<Vec<_>>(),
            "favorite": self.model.favorite.iter().map(|m| serde_json::json!({ "providerID": m.provider_id, "modelID": m.model_id })).collect::<Vec<_>>(),
            "variant": self.model.variant,
        });
        tokio::spawn(async move {
            let _ = crate::util::persistence::write_json_atomic(&file, &payload).await;
        });
    }

    fn save_session(&mut self) {
        if !self.session.ready {
            self.session_pending = true;
            return;
        }
        self.session_pending = false;
        let file = self.session_file.clone();
        let payload = serde_json::json!({ "pinned": self.session.pinned });
        tokio::spawn(async move {
            let _ = crate::util::persistence::write_json_atomic(&file, &payload).await;
        });
    }

    // -- agents -----------------------------------------------------------

    /// Non-subagent, visible agents (mirrors the `agents` memo).
    pub fn agents<'a>(&self, sync: &'a SyncStore) -> Vec<&'a Value> {
        sync.agent
            .iter()
            .filter(|a| {
                a.get("mode").and_then(|v| v.as_str()) != Some("subagent")
                    && a.get("hidden").and_then(|v| v.as_bool()) != Some(true)
            })
            .collect()
    }

    pub fn visible_agents<'a>(&self, sync: &'a SyncStore) -> Vec<&'a Value> {
        sync.agent
            .iter()
            .filter(|a| a.get("hidden").and_then(|v| v.as_bool()) != Some(true))
            .collect()
    }

    /// Mirrors `agent.current()` (named current or first).
    pub fn agent_current<'a>(&self, sync: &'a SyncStore) -> Option<&'a Value> {
        let agents = self.agents(sync);
        match self.agent.current.as_deref() {
            Some(name) => agents
                .iter()
                .find(|a| a.get("name").and_then(|v| v.as_str()) == Some(name))
                .copied()
                .or_else(|| agents.into_iter().next()),
            None => agents.into_iter().next(),
        }
    }

    pub fn agent_set(&mut self, sync: &SyncStore, name: &str, toast: &mut dyn LocalToast) {
        if !self
            .agents(sync)
            .iter()
            .any(|a| a.get("name").and_then(|v| v.as_str()) == Some(name))
        {
            toast.show(
                ToastVariant::Warning,
                &format!("Agent not found: {name}"),
                TOAST_DURATION_MS,
            );
            return;
        }
        self.agent.current = Some(name.to_string());
    }

    pub fn agent_move(&mut self, sync: &SyncStore, direction: i64) {
        let agents = self.agents(sync);
        let current = match self.agent_current(sync) {
            Some(current) => current,
            None => return,
        };
        let name = current.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let mut next = agents
            .iter()
            .position(|a| a.get("name").and_then(|v| v.as_str()) == Some(name))
            .unwrap_or(0) as i64
            + direction;
        if next < 0 {
            next = agents.len() as i64 - 1;
        }
        if next >= agents.len() as i64 {
            next = 0;
        }
        if let Some(agent) = agents.get(next as usize) {
            if let Some(name) = agent.get("name").and_then(|v| v.as_str()) {
                self.agent.current = Some(name.to_string());
            }
        }
    }

    /// Mirrors `agent.color(name)` (7-color cycle, explicit hex, theme keys).
    pub fn agent_color(&self, theme: &Theme, sync: &SyncStore, name: &str) -> Rgba {
        let colors = [
            theme.secondary,
            theme.accent,
            theme.success,
            theme.warning,
            theme.primary,
            theme.error,
            theme.info,
        ];
        let visible = self.visible_agents(sync);
        let index = match visible
            .iter()
            .position(|a| a.get("name").and_then(|v| v.as_str()) == Some(name))
        {
            Some(index) => index,
            None => return colors[0],
        };
        let agent = visible[index];
        if let Some(color) = agent.get("color").and_then(|v| v.as_str()) {
            if color.starts_with('#') {
                return Rgba::from_hex(color);
            }
            return theme_named_color(theme, color);
        }
        colors[index % colors.len()]
    }

    // -- models -----------------------------------------------------------

    pub fn is_model_valid(&self, sync: &SyncStore, model: &ModelId) -> bool {
        sync.provider
            .iter()
            .find(|p| p.get("id").and_then(|v| v.as_str()) == Some(model.provider_id.as_str()))
            .and_then(|p| p.get("models"))
            .and_then(|m| m.get(model.model_id.as_str()))
            .is_some()
    }

    fn first_valid(&self, sync: &SyncStore, candidates: Vec<Option<ModelId>>) -> Option<ModelId> {
        candidates
            .into_iter()
            .flatten()
            .find(|m| self.is_model_valid(sync, m))
    }

    /// Mirrors the `fallbackModel` memo (args → config → recent → default).
    pub fn fallback_model(
        &self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
    ) -> Option<ModelId> {
        if let Some(arg) = args_model {
            let parsed = parse_model(arg);
            if self.is_model_valid(sync, &parsed) {
                return Some(parsed);
            }
        }
        if let Some(config) = config_model {
            let parsed = parse_model(config);
            if self.is_model_valid(sync, &parsed) {
                return Some(parsed);
            }
        }
        for item in &self.model.recent {
            if self.is_model_valid(sync, item) {
                return Some(item.clone());
            }
        }
        let provider = sync.provider.first()?;
        let provider_id = provider.get("id")?.as_str()?;
        let default_model = sync.provider_default.get(provider_id).cloned();
        let first_model = provider.get("models")?.as_object()?.keys().next().cloned();
        let model = default_model.or(first_model)?;
        Some(ModelId {
            provider_id: provider_id.to_string(),
            model_id: model,
        })
    }

    /// Mirrors `currentModel` (per-agent override → agent default → fallback).
    pub fn current_model(
        &self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
    ) -> Option<ModelId> {
        let current = self.agent_current(sync);
        let agent_name = current.and_then(|a| a.get("name")).and_then(|v| v.as_str());
        let per_agent = agent_name
            .and_then(|name| self.model.per_agent.get(name))
            .cloned();
        let agent_default = current.and_then(|a| a.get("model")).and_then(|m| {
            Some(ModelId {
                provider_id: m.get("providerID")?.as_str()?.to_string(),
                model_id: m.get("modelID")?.as_str()?.to_string(),
            })
        });
        self.first_valid(
            sync,
            vec![
                per_agent,
                agent_default,
                self.fallback_model(sync, args_model, config_model),
            ],
        )
    }

    /// Mirrors the `parsed` memo (provider/model display names + reasoning).
    pub fn model_parsed(
        &self,
        sync: &SyncStore,
        current: Option<&ModelId>,
    ) -> (String, String, bool) {
        let Some(current) = current else {
            return (
                "Connect a provider".to_string(),
                "No provider selected".to_string(),
                false,
            );
        };
        let provider = sync
            .provider
            .iter()
            .find(|p| p.get("id").and_then(|v| v.as_str()) == Some(current.provider_id.as_str()));
        let info = provider
            .and_then(|p| p.get("models"))
            .and_then(|m| m.get(current.model_id.as_str()));
        (
            provider
                .and_then(|p| p.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or(current.provider_id.as_str())
                .to_string(),
            info.and_then(|i| i.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or(current.model_id.as_str())
                .to_string(),
            info.and_then(|i| i.get("capabilities"))
                .and_then(|c| c.get("reasoning"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        )
    }

    pub fn model_cycle(
        &mut self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
        direction: i64,
    ) {
        let current = match self.current_model(sync, args_model, config_model) {
            Some(current) => current,
            None => return,
        };
        let index = match self.model.recent.iter().position(|x| x == &current) {
            Some(index) => index,
            None => return,
        };
        let mut next = index as i64 + direction;
        if next < 0 {
            next = self.model.recent.len() as i64 - 1;
        }
        if next >= self.model.recent.len() as i64 {
            next = 0;
        }
        let Some(value) = self.model.recent.get(next as usize).cloned() else {
            return;
        };
        let Some(agent) = self
            .agent_current(sync)
            .and_then(|a| a.get("name"))
            .and_then(|v| v.as_str())
            .map(str::to_string)
        else {
            return;
        };
        self.model.per_agent.insert(agent, value);
    }

    pub fn model_cycle_favorite(
        &mut self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
        direction: i64,
        toast: &mut dyn LocalToast,
    ) {
        let favorites: Vec<ModelId> = self
            .model
            .favorite
            .iter()
            .filter(|item| self.is_model_valid(sync, item))
            .cloned()
            .collect();
        if favorites.is_empty() {
            toast.show(
                ToastVariant::Info,
                "Add a favorite model to use this shortcut",
                TOAST_DURATION_MS,
            );
            return;
        }
        let current = self.current_model(sync, args_model, config_model);
        let mut index: i64 = -1;
        if let Some(current) = current {
            if let Some(found) = favorites.iter().position(|x| x == &current) {
                index = found as i64;
            }
        }
        if index == -1 {
            index = if direction == 1 {
                0
            } else {
                favorites.len() as i64 - 1
            };
        } else {
            index += direction;
            if index < 0 {
                index = favorites.len() as i64 - 1;
            }
            if index >= favorites.len() as i64 {
                index = 0;
            }
        }
        let Some(next) = favorites.get(index as usize).cloned() else {
            return;
        };
        let Some(agent) = self
            .agent_current(sync)
            .and_then(|a| a.get("name"))
            .and_then(|v| v.as_str())
            .map(str::to_string)
        else {
            return;
        };
        self.model.per_agent.insert(agent, next.clone());
        self.model.recent = recent_models(next, &self.model.recent);
        self.save_model();
    }

    pub fn model_set(
        &mut self,
        sync: &SyncStore,
        model: ModelId,
        recent: bool,
        toast: &mut dyn LocalToast,
    ) {
        if !self.is_model_valid(sync, &model) {
            toast.show(
                ToastVariant::Warning,
                &format!(
                    "Model {}/{} is not valid",
                    model.provider_id, model.model_id
                ),
                TOAST_DURATION_MS,
            );
            return;
        }
        let Some(agent) = self
            .agent_current(sync)
            .and_then(|a| a.get("name"))
            .and_then(|v| v.as_str())
            .map(str::to_string)
        else {
            return;
        };
        self.model.per_agent.insert(agent, model.clone());
        if recent {
            self.model.recent = recent_models(model, &self.model.recent);
            self.save_model();
        }
    }

    pub fn model_toggle_favorite(
        &mut self,
        sync: &SyncStore,
        model: ModelId,
        toast: &mut dyn LocalToast,
    ) {
        if !self.is_model_valid(sync, &model) {
            toast.show(
                ToastVariant::Warning,
                &format!(
                    "Model {}/{} is not valid",
                    model.provider_id, model.model_id
                ),
                TOAST_DURATION_MS,
            );
            return;
        }
        if self.model.favorite.contains(&model) {
            self.model.favorite.retain(|x| x != &model);
        } else {
            self.model.favorite.insert(0, model);
        }
        self.save_model();
    }

    fn variant_key(
        &self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
    ) -> Option<String> {
        let current = self.current_model(sync, args_model, config_model)?;
        Some(format!("{}/{}", current.provider_id, current.model_id))
    }

    pub fn variant_selected(
        &self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
    ) -> Option<String> {
        let key = self.variant_key(sync, args_model, config_model)?;
        self.model.variant.get(&key).cloned()
    }

    pub fn variant_current(
        &self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
    ) -> Option<String> {
        let selected = self.variant_selected(sync, args_model, config_model)?;
        if !self
            .variant_list(sync, args_model, config_model)
            .contains(&selected)
        {
            return None;
        }
        Some(selected)
    }

    pub fn variant_list(
        &self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
    ) -> Vec<String> {
        let Some(current) = self.current_model(sync, args_model, config_model) else {
            return Vec::new();
        };
        let info = sync
            .provider
            .iter()
            .find(|p| p.get("id").and_then(|v| v.as_str()) == Some(current.provider_id.as_str()))
            .and_then(|p| p.get("models"))
            .and_then(|m| m.get(current.model_id.as_str()));
        info.and_then(|i| i.get("variants"))
            .and_then(|v| v.as_object())
            .map(|map| map.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn variant_set(
        &mut self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
        value: Option<String>,
    ) {
        let Some(key) = self.variant_key(sync, args_model, config_model) else {
            return;
        };
        self.model
            .variant
            .insert(key, value.unwrap_or_else(|| "default".to_string()));
        self.save_model();
    }

    pub fn variant_cycle(
        &mut self,
        sync: &SyncStore,
        args_model: Option<&str>,
        config_model: Option<&str>,
    ) {
        let variants = self.variant_list(sync, args_model, config_model);
        if variants.is_empty() {
            return;
        }
        let Some(current) = self.variant_current(sync, args_model, config_model) else {
            self.variant_set(sync, args_model, config_model, variants.first().cloned());
            return;
        };
        match variants.iter().position(|v| v == &current) {
            Some(index) if index + 1 < variants.len() => self.variant_set(
                sync,
                args_model,
                config_model,
                variants.get(index + 1).cloned(),
            ),
            _ => self.variant_set(sync, args_model, config_model, None),
        }
    }

    // -- sessions ---------------------------------------------------------

    /// Pinned slots among top-level sessions, capped at 9 (mirrors `slots`).
    pub fn slots(&self, sync: &SyncStore) -> Vec<String> {
        let existing: std::collections::HashSet<&str> = sync
            .session
            .iter()
            .filter(|s| s.get("parentID").is_none())
            .filter_map(|s| s.get("id").and_then(|v| v.as_str()))
            .collect();
        self.session
            .pinned
            .iter()
            .filter(|id| existing.contains(id.as_str()))
            .take(9)
            .cloned()
            .collect()
    }

    pub fn is_pinned(&self, session_id: &str) -> bool {
        self.session.pinned.iter().any(|id| id == session_id)
    }

    pub fn toggle_pin(&mut self, session_id: &str) {
        if self.is_pinned(session_id) {
            self.session.pinned.retain(|x| x != session_id);
        } else {
            self.session.pinned.push(session_id.to_string());
        }
        self.save_session();
    }

    /// Mirrors `prune` (also persists, verbatim).
    pub fn prune(&mut self, session_id: &str) {
        if self.is_pinned(session_id) {
            self.session.pinned.retain(|x| x != session_id);
        }
        self.save_session();
    }

    pub fn quick_switch(&self, sync: &SyncStore, route: &mut RouteStore, slot: usize) {
        let Some(target) = self.slots(sync).get(slot.saturating_sub(1)).cloned() else {
            return;
        };
        if matches!(route.data(), Route::Session(session) if session.session_id == target) {
            return;
        }
        route.navigate(Route::Session(crate::context::route::SessionRoute {
            session_id: target,
            prompt: None,
        }));
    }

    // -- MCP --------------------------------------------------------------

    pub fn mcp_is_enabled(&self, sync: &SyncStore, name: &str) -> bool {
        sync.mcp
            .get(name)
            .and_then(|s| s.get("status"))
            .and_then(|v| v.as_str())
            == Some("connected")
    }

    pub async fn mcp_toggle(
        &self,
        client: &Arc<dyn SdkClient>,
        sync: &SyncStore,
        name: &str,
    ) -> Result<(), String> {
        if self.mcp_is_enabled(sync, name) {
            client
                .call("mcp.disconnect", serde_json::json!({ "name": name }))
                .await
                .map(|_| ())
        } else {
            client
                .call("mcp.connect", serde_json::json!({ "name": name }))
                .await
                .map(|_| ())
        }
    }

    /// Mirrors the agent-model validity effect (returns the warning text).
    pub fn agent_model_warning(&self, sync: &SyncStore) -> Option<String> {
        let agent = self.agent_current(sync)?;
        let model = agent.get("model")?;
        let parsed = ModelId {
            provider_id: model.get("providerID")?.as_str()?.to_string(),
            model_id: model.get("modelID")?.as_str()?.to_string(),
        };
        if self.is_model_valid(sync, &parsed) {
            return None;
        }
        Some(format!(
            "Agent {}'s configured model {}/{} is not valid",
            agent.get("name").and_then(|v| v.as_str()).unwrap_or(""),
            parsed.provider_id,
            parsed.model_id
        ))
    }
}
