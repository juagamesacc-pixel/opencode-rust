//! Rust port of `packages/app/src/pages/layout/helpers.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `layout/helpers.ts` -> `layout/helpers.rs` (kebab -> snake_case).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub directory: String,
    pub parent_id: Option<String>,
    pub project_id: Option<String>,
    pub time: SessionTime,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTime {
    pub created: i64,
    pub updated: Option<i64>,
    pub archived: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SessionStore {
    pub session: Option<Vec<Session>>,
    pub directory: String,
}

fn path_key(s: &str) -> String {
    s.trim_end_matches('/').to_string()
}

pub fn compare_session_time(a: &Session, b: &Session) -> std::cmp::Ordering {
    let a_time = a.time.updated.unwrap_or(a.time.created);
    let b_time = b.time.updated.unwrap_or(b.time.created);
    let ord = b_time.cmp(&a_time);
    if ord != std::cmp::Ordering::Equal {
        return ord;
    }
    a.id.cmp(&b.id)
}

fn is_root_visible_session(session: &Session, directory: &str) -> bool {
    path_key(&session.directory) == path_key(directory)
        && session.parent_id.is_none()
        && session.time.archived.is_none()
}

pub fn roots(store: &SessionStore) -> Vec<Session> {
    store
        .session
        .clone()
        .unwrap_or_default()
        .into_iter()
        .filter(|s| is_root_visible_session(s, &store.directory))
        .collect()
}

pub fn sorted_root_sessions(store: &SessionStore, _now: i64) -> Vec<Session> {
    let mut r = roots(store);
    r.sort_by(compare_session_time);
    r
}

pub fn latest_root_session(stores: &[SessionStore], _now: i64) -> Option<Session> {
    let mut all: Vec<Session> = stores.iter().flat_map(roots).collect();
    all.sort_by(compare_session_time);
    all.into_iter().next()
}

pub fn has_project_permissions<T>(
    request: Option<&std::collections::HashMap<String, Option<Vec<T>>>>,
    include: impl Fn(&T) -> bool,
) -> bool {
    if let Some(map) = request {
        for v in map.values().flatten() {
            if v.iter().any(&include) {
                return true;
            }
        }
    }
    false
}

pub fn child_session_on_path(
    sessions: Option<&[Session]>,
    root_id: &str,
    active_id: Option<&str>,
) -> Option<Session> {
    let active = active_id?;
    if active == root_id {
        return None;
    }
    let map: std::collections::HashMap<&str, &Session> = sessions
        .unwrap_or(&[])
        .iter()
        .map(|s| (s.id.as_str(), s))
        .collect();
    let mut id = active;
    loop {
        let session = map.get(id)?;
        let parent = session.parent_id.as_deref()?;
        if parent == root_id {
            return Some((*session).clone());
        }
        id = parent;
    }
}

pub fn display_name(project: &serde_json::Value) -> String {
    if let Some(name) = project.get("name").and_then(|v| v.as_str()) {
        if !name.is_empty() {
            return name.to_string();
        }
    }
    if let Some(worktree) = project.get("worktree").and_then(|v| v.as_str()) {
        let filename = worktree.rsplit('/').next().unwrap_or(worktree);
        if !filename.is_empty() {
            return filename.to_string();
        }
        return worktree.to_string();
    }
    String::new()
}

pub fn get_project_avatar_source(
    id: Option<&str>,
    icon: Option<&serde_json::Value>,
) -> Option<String> {
    const OPENCODE_PROJECT_ID: &str = "4b0ea68d7af9a6031a7ffda7ad66e0cb83315750";
    if id == Some(OPENCODE_PROJECT_ID) {
        return Some("https://opencode.ai/favicon.svg".to_string());
    }
    if let Some(ic) = icon {
        if let Some(ov) = ic.get("override").and_then(|v| v.as_str()) {
            if !ov.is_empty() {
                return Some(ov.to_string());
            }
        }
        if ic.get("color").and_then(|v| v.as_str()).is_some() {
            return None;
        }
        if let Some(url) = ic.get("url").and_then(|v| v.as_str()) {
            return Some(url.to_string());
        }
    }
    None
}

pub fn error_message(err: &serde_json::Value, fallback: &str) -> String {
    if let Some(data) = err
        .get("data")
        .and_then(|d| d.get("message"))
        .and_then(|v| v.as_str())
    {
        return data.to_string();
    }
    if let Some(msg) = err.get("message").and_then(|v| v.as_str()) {
        return msg.to_string();
    }
    fallback.to_string()
}

pub fn effective_workspace_order(
    local: &str,
    dirs: &[String],
    persisted: Option<&[String]>,
) -> Vec<String> {
    let root = path_key(local);
    let mut live: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for dir in dirs {
        let key = path_key(dir);
        if key == root {
            continue;
        }
        live.entry(key.clone()).or_insert_with(|| dir.clone());
    }
    if persisted.is_none() || persisted.unwrap().is_empty() {
        let mut out = vec![local.to_string()];
        out.extend(live.values().cloned());
        return out;
    }
    let mut result = vec![local.to_string()];
    for dir in persisted.unwrap() {
        let key = path_key(dir);
        if key == root {
            continue;
        }
        if let Some(m) = live.remove(&key) {
            result.push(m);
        }
    }
    result.extend(live.values().cloned());
    result
}

pub fn toggle_home_project_selection(
    current: Option<serde_json::Value>,
    server: &str,
    directory: &str,
) -> serde_json::Value {
    if let Some(cur) = &current {
        if cur.get("server").and_then(|v| v.as_str()) == Some(server)
            && cur.get("directory").and_then(|v| v.as_str()) == Some(directory)
        {
            return serde_json::json!({ "server": server });
        }
    }
    serde_json::json!({ "server": server, "directory": directory })
}
