//! Rust port of `packages/core/src/tool/application-tools.ts`.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Entry {
    pub identity: String,
    pub tool_name: String,
}

#[derive(Debug, Default, Clone)]
pub struct ApplicationTools {
    entries: Arc<RwLock<HashMap<String, Entry>>>,
}

impl ApplicationTools {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, tools: HashMap<String, String>) -> Result<(), String> {
        for name in tools.keys() {
            crate::tool::tool::validate_name(name).map_err(|e| e.message)?;
        }
        let mut guard = self.entries.write().unwrap();
        for (name, tool) in tools {
            guard.insert(
                name.clone(),
                Entry {
                    identity: format!("id:{name}"),
                    tool_name: tool,
                },
            );
        }
        Ok(())
    }

    pub fn entries(&self) -> HashMap<String, Entry> {
        self.entries.read().unwrap().clone()
    }
}

// PROVISIONAL pending State.create + Scope finalizers — sync RwLock preserves latest-wins + retrieval.
