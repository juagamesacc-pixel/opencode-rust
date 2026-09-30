//! Rust port of `packages/app/src/pages/session/composer/session-request-tree.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/composer/session-request-tree.ts` -> `session/composer/session_request_tree.rs` (kebab -> snake_case).

use std::collections::{HashMap, HashSet};

fn session_tree_request<T: Clone>(
    sessions: &[Session],
    request: &HashMap<String, Vec<T>>,
    session_id: Option<&str>,
    include: impl Fn(&T) -> bool,
) -> Option<T> {
    let sid = session_id?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in sessions {
        if let Some(parent) = &s.parent_id {
            map.entry(parent.clone()).or_default().push(s.id.clone());
        }
    }
    let mut seen: HashSet<String> = HashSet::new();
    let mut ids: Vec<String> = vec![sid.to_string()];
    seen.insert(sid.to_string());
    let mut idx = 0;
    while idx < ids.len() {
        let id = ids[idx].clone();
        idx += 1;
        if let Some(children) = map.get(&id) {
            for child in children {
                if seen.contains(child) {
                    continue;
                }
                seen.insert(child.clone());
                ids.push(child.clone());
            }
        }
    }
    for id in ids {
        if let Some(list) = request.get(&id) {
            if let Some(found) = list.iter().find(|item| include(item)) {
                return Some(found.clone());
            }
        }
    }
    None
}

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub id: String,
    pub parent_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PermissionRequest {
    pub id: String,
    pub session_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuestionRequest {
    pub id: String,
    pub session_id: String,
}

pub fn session_permission_request(
    sessions: &[Session],
    request: &HashMap<String, Vec<PermissionRequest>>,
    session_id: Option<&str>,
    include: Option<fn(&PermissionRequest) -> bool>,
) -> Option<PermissionRequest> {
    let f = include.unwrap_or(|_| true);
    session_tree_request(sessions, request, session_id, f)
}

pub fn session_question_request(
    sessions: &[Session],
    request: &HashMap<String, Vec<QuestionRequest>>,
    session_id: Option<&str>,
) -> Option<QuestionRequest> {
    session_tree_request(sessions, request, session_id, |_| true)
}
