//! Rust port of `packages/app/src/pages/session/handoff.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/handoff.ts` -> `session/handoff.rs` (kebab -> snake_case).

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Debug, PartialEq)]
pub struct HandoffSession {
    pub prompt: String,
    pub files: HashMap<String, Option<serde_json::Value>>,
}

static SESSION_STORE: OnceLock<Mutex<HashMap<String, HandoffSession>>> = OnceLock::new();
static TERMINAL_STORE: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();
const MAX: usize = 40;

fn session_store() -> &'static Mutex<HashMap<String, HandoffSession>> {
    SESSION_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}
fn terminal_store() -> &'static Mutex<HashMap<String, Vec<String>>> {
    TERMINAL_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn touch<K: Eq + std::hash::Hash + Clone, V: Clone>(map: &mut HashMap<K, V>, key: K, value: V) {
    map.remove(&key);
    map.insert(key.clone(), value);
    while map.len() > MAX {
        if let Some(first) = map.keys().next().cloned() {
            map.remove(&first);
        } else {
            break;
        }
    }
}

pub fn set_session_handoff(key: &str, patch: HandoffSession) {
    let mut m = session_store().lock().unwrap();
    let prev = m.get(key).cloned().unwrap_or(HandoffSession {
        prompt: String::new(),
        files: HashMap::new(),
    });
    let merged = HandoffSession {
        prompt: if patch.prompt.is_empty() {
            prev.prompt
        } else {
            patch.prompt
        },
        files: if patch.files.is_empty() {
            prev.files
        } else {
            patch.files
        },
    };
    touch(&mut *m, key.to_string(), merged);
}
pub fn get_session_handoff(key: &str) -> Option<HandoffSession> {
    session_store().lock().unwrap().get(key).cloned()
}
pub fn set_terminal_handoff(key: &str, value: Vec<String>) {
    let mut m = terminal_store().lock().unwrap();
    touch(&mut *m, key.to_string(), value);
}
pub fn get_terminal_handoff(key: &str) -> Option<Vec<String>> {
    terminal_store().lock().unwrap().get(key).cloned()
}
