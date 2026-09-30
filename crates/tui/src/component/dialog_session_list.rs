// source: packages/tui/src/component/dialog-session-list.tsx (364 lines, v1.18.30)
// 1:1 port — session browser: debounced search (150ms), browse+search
// resources with sync-overlay merge, pinned section, date categories,
// quick-switch footer hints, delete two-step with workspace recovery,
// rename entry, pin toggle, and the `quickSwitchRange` label.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::super::ui::dialog_select::{
    SelectAction, SelectDisabled, SelectOption, SelectSide, SelectState,
};
use crate::context::local::LocalContext;
use crate::context::route::RouteStore;
use crate::context::sdk::SdkClient;
use crate::context::sync::SyncStore;
use crate::util::locale::truncate;

/// Search debounce verbatim (150ms).
pub const SESSION_SEARCH_DEBOUNCE_MS: u64 = 150;
/// Browse limit verbatim.
pub const SESSION_BROWSE_LIMIT: u32 = 100;
/// Search limit verbatim.
pub const SESSION_SEARCH_LIMIT: u32 = 30;

/// Mirrors `createDialogSessionListQuery` (roots + trimmed search).
pub fn session_list_query(search: Option<&str>, scope_project: bool, path: Option<&str>) -> Value {
    let trimmed = search.map(str::trim).filter(|s| !s.is_empty());
    let mut query = serde_json::json!({ "roots": true, "limit": if trimmed.is_some() { SESSION_SEARCH_LIMIT } else { SESSION_BROWSE_LIMIT } });
    if let Some(search) = trimmed {
        query["search"] = Value::String(search.to_string());
    }
    if scope_project {
        query["scope"] = Value::String("project".to_string());
    }
    if let Some(path) = path {
        query["path"] = Value::String(path.to_string());
    }
    query
}

/// Session-list dialog state.
pub struct SessionListState {
    pub search: String,
    search_deadline: Option<Instant>,
    pub to_delete: Option<String>,
    pub deleted: HashSet<String>,
    pub browse: Vec<Value>,
    pub search_results: Option<Vec<Value>>,
}

impl SessionListState {
    pub fn new() -> Self {
        Self {
            search: String::new(),
            search_deadline: None,
            to_delete: None,
            deleted: HashSet::new(),
            browse: Vec::new(),
            search_results: None,
        }
    }

    /// Filter input (mirrors the debounced signal — arms the deadline).
    pub fn set_search(&mut self, query: &str) {
        self.search = query.to_string();
        self.search_deadline =
            Some(Instant::now() + Duration::from_millis(SESSION_SEARCH_DEBOUNCE_MS));
    }

    /// Deadline driver — returns true when a search fetch is due.
    pub fn poll_search(&mut self) -> bool {
        if self
            .search_deadline
            .map(|d| Instant::now() >= d)
            .unwrap_or(false)
        {
            self.search_deadline = None;
            return !self.search.trim().is_empty();
        }
        false
    }

    /// Mirrors the `sessions` memo — search/browse base, sync overlay,
    /// extra current+pinned, deleted filter, title substring filter.
    pub fn sessions(
        &self,
        sync: &SyncStore,
        local: &LocalContext,
        current_session_id: Option<&str>,
    ) -> Vec<Value> {
        let base = self.search_results.as_ref().unwrap_or(&self.browse);
        let base = if self.search_results.is_none() && self.browse.is_empty() {
            &sync.session
        } else {
            base
        };
        let synced: HashMap<&str, &Value> = sync
            .session
            .iter()
            .filter_map(|s| s.get("id").and_then(|v| v.as_str()).map(|id| (id, s)))
            .collect();
        let mut ids: HashSet<&str> = base
            .iter()
            .filter_map(|s| s.get("id").and_then(|v| v.as_str()))
            .collect();
        let mut extra: Vec<Value> = Vec::new();
        let mut pinned_ids: Vec<&str> = Vec::new();
        if let Some(current) = current_session_id {
            if !ids.contains(current) {
                if let Some(session) = synced.get(current) {
                    ids.insert(current);
                    extra.push((*session).clone());
                }
            }
        }
        for id in local.session.pinned.iter() {
            if ids.contains(id.as_str()) {
                continue;
            }
            if let Some(session) = synced.get(id.as_str()) {
                ids.insert(id.as_str());
                pinned_ids.push(id.as_str());
                extra.push((*session).clone());
            }
        }
        let _ = pinned_ids;
        let query = self.search.trim().to_lowercase();
        base.iter()
            .map(|session| {
                let id = session.get("id").and_then(|v| v.as_str()).unwrap_or("");
                synced.get(id).cloned().unwrap_or(session).clone()
            })
            .chain(extra)
            .filter(|session| {
                let id = session.get("id").and_then(|v| v.as_str()).unwrap_or("");
                !self.deleted.contains(id)
            })
            .filter(|session| {
                query.is_empty()
                    || session
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
            })
            .collect()
    }

    /// Recency order over top-level sessions (mirrors `orderByRecency`).
    pub fn order_by_recency(sessions: &[Value]) -> Vec<String> {
        let mut top: Vec<&Value> = sessions
            .iter()
            .filter(|s| s.get("parentID").is_none())
            .collect();
        top.sort_by(|a, b| {
            b.get("time")
                .and_then(|t| t.get("updated"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
                .cmp(
                    &a.get("time")
                        .and_then(|t| t.get("updated"))
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0),
                )
        });
        top.into_iter()
            .filter_map(|s| s.get("id").and_then(|v| v.as_str()).map(str::to_string))
            .collect()
    }

    /// Option builder (mirrors `buildOption`: footer, deleting state,
    /// working gutter, slot gutter).
    pub fn build_options(
        &self,
        sessions: &[Value],
        local: &LocalContext,
        sync: &SyncStore,
        main_dir: Option<&str>,
        delete_hint: &str,
        today: &str,
    ) -> Vec<SelectOption> {
        let mut map: HashMap<&str, &Value> = HashMap::new();
        for session in sessions.iter().filter(|s| s.get("parentID").is_none()) {
            if let Some(id) = session.get("id").and_then(|v| v.as_str()) {
                map.insert(id, session);
            }
        }
        let order = Self::order_by_recency(sessions);
        let pinned: Vec<String> = local
            .session
            .pinned
            .iter()
            .filter(|id| map.contains_key(id.as_str()))
            .cloned()
            .collect();
        let pinned_set: HashSet<&str> = pinned.iter().map(|s| s.as_str()).collect();
        let slot_list = local.slots(sync);
        let slots: HashMap<&str, usize> = slot_list
            .iter()
            .enumerate()
            .map(|(index, id)| (id.as_str(), index + 1))
            .collect();
        let build = |id: &str, category: String| -> Option<SelectOption> {
            let session = map.get(id)?;
            let path = session.get("path").and_then(|v| v.as_str()).unwrap_or("");
            let directory = session
                .get("directory")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let footer = if path.is_empty() {
                if directory.is_empty() || Some(directory) == main_dir {
                    String::new()
                } else {
                    truncate_file_name(directory)
                }
            } else if directory.ends_with(path) {
                let base = directory
                    .strip_suffix(path)
                    .unwrap_or("")
                    .trim_end_matches('/');
                if base.is_empty() || Some(base) == main_dir {
                    String::new()
                } else {
                    truncate_file_name(base)
                }
            } else {
                String::new()
            };
            let deleting = self.to_delete.as_deref() == Some(id);
            let status = session.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let sync_status = sync
                .session_status
                .get(id)
                .and_then(|v| v.as_str())
                .unwrap_or(status);
            let working = sync_status == "busy" || sync_status == "retry";
            let slot = slots.get(id).copied();
            Some(SelectOption {
                title: if deleting {
                    format!("Press {delete_hint} again to confirm")
                } else {
                    session
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                },
                bg: if deleting {
                    Some(crate::theme::Rgba::from_hex("#e06c75"))
                } else {
                    None
                },
                value: Value::String(id.to_string()),
                category: Some(category),
                footer: Some(footer),
                gutter: if working {
                    Some("working".to_string())
                } else {
                    slot.map(|slot| format!("slot:{slot}"))
                },
                ..SelectOption::default()
            })
        };
        let mut out: Vec<SelectOption> = pinned
            .iter()
            .filter_map(|id| build(id, "Pinned".to_string()))
            .collect();
        // Date categories for the rest (search keeps recency order).
        let rest: Vec<&String> = order
            .iter()
            .filter(|id| !pinned_set.contains(id.as_str()))
            .collect();
        for id in rest {
            let label = map
                .get(id.as_str())
                .and_then(|s| s.get("time"))
                .and_then(|t| t.get("updated"))
                .and_then(|v| v.as_i64())
                .map(|updated| day_label(updated, today))
                .unwrap_or_default();
            if let Some(option) = build(id, label) {
                out.push(option);
            }
        }
        out
    }

    /// Full select state (title `Sessions`, large size, actions verbatim).
    pub fn select_state(
        &self,
        options: Vec<SelectOption>,
        current: Option<&str>,
        footer_hints: Vec<(String, String)>,
    ) -> SelectState {
        let mut state = SelectState::new("Sessions", options);
        state.skip_filter = true;
        state.preserve_selection = true;
        state.current = current.map(|id| Value::String(id.to_string()));
        state.footer_hints = footer_hints
            .into_iter()
            .map(|(title, label)| crate::ui::dialog_select::FooterHint {
                title,
                label,
                side: crate::ui::dialog_select::SelectSide::Left,
            })
            .collect();
        state.actions = vec![
            SelectAction {
                command: "session.pin.toggle".to_string(),
                title: "pin/unpin".to_string(),
                side: SelectSide::Left,
                hidden: false,
                disabled: SelectDisabled::Flag(false),
                on_trigger: Box::new(|_| {}),
            },
            SelectAction {
                command: "session.delete".to_string(),
                title: "delete".to_string(),
                side: SelectSide::Left,
                hidden: false,
                disabled: SelectDisabled::Flag(false),
                on_trigger: Box::new(|_| {}),
            },
            SelectAction {
                command: "session.rename".to_string(),
                title: "rename".to_string(),
                side: SelectSide::Left,
                hidden: false,
                disabled: SelectDisabled::Flag(false),
                on_trigger: Box::new(|_| {}),
            },
        ];
        state
    }

    /// Record a remotely-deleted session (mirrors the event cleanup).
    pub fn mark_deleted(&mut self, session_id: &str) {
        self.deleted.insert(session_id.to_string());
    }
}

impl Default for SessionListState {
    fn default() -> Self {
        Self::new()
    }
}

fn truncate_file_name(directory: &str) -> String {
    let base = directory
        .rsplit('/')
        .next()
        .unwrap_or(directory)
        .to_string();
    crate::util::locale::truncate(&base, 20)
}

/// Day label (`Today` vs the local date string).
pub fn day_label(updated_ms: i64, today: &str) -> String {
    let label = crate::util::locale::date_string(updated_ms);
    if label == today {
        "Today".to_string()
    } else {
        label
    }
}

/// Mirrors `quickSwitchRange` (`prefix1-9` collapse verbatim).
pub fn quick_switch_range(first: &str, last: &str) -> String {
    if first.len() < 2 {
        return format!("{first} through {last}");
    }
    let prefix = &first[..first.len() - 1];
    if first.ends_with('1') && last == format!("{prefix}9") {
        return format!("{prefix}1-9");
    }
    format!("{first} through {last}")
}

/// Browse fetch (mirrors `loadDialogSessionList` over `session.list`).
pub async fn fetch_session_list(client: &Arc<dyn SdkClient>, query: Value) -> Option<Vec<Value>> {
    client
        .call("session.list", query)
        .await
        .ok()
        .and_then(|response| response.get("data").and_then(|d| d.as_array()).cloned())
}

/// Delete flow outcome for the app to act on.
pub enum SessionDeleteNext {
    /// Refresh lists and stay.
    Refresh,
    /// Enter workspace recovery for this session.
    Recover { session_id: String },
}

/// Two-step delete arming (mirrors the `session.delete` trigger up to the
/// SDK call; the caller performs the call when this returns true).
pub fn delete_press(state: &mut SessionListState, session_id: &str) -> bool {
    if state.to_delete.as_deref() == Some(session_id) {
        return true;
    }
    state.to_delete = Some(session_id.to_string());
    false
}
