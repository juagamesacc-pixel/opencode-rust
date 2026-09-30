//! Rust port of `packages/app/src/context/local-agent.ts` (opencode v1.18.30).
//!
//! Source 7 lines. Exports: `hasCustomAgent`, `resolveAgent`.
//! 1:1 faithful — same names, behavior, edge cases.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentItem {
    pub native: Option<bool>,
    pub name: Option<String>,
}

#[allow(non_snake_case)]
pub fn hasCustomAgent(items: &[AgentItem]) -> bool {
    items.iter().any(|item| item.native == Some(false))
}

pub trait AgentNamed {
    fn agent_name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedAgent {
    pub name: String,
}

impl AgentNamed for NamedAgent {
    fn agent_name(&self) -> &str {
        &self.name
    }
}

impl AgentNamed for AgentItem {
    fn agent_name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

#[allow(non_snake_case)]
pub fn resolveAgent<T: AgentNamed + Clone>(items: &[T], name: Option<&str>) -> Option<T> {
    if let Some(n) = name {
        if let Some(found) = items.iter().find(|item| item.agent_name() == n) {
            return Some(found.clone());
        }
    }
    if let Some(found) = items.iter().find(|item| item.agent_name() == "build") {
        return Some(found.clone());
    }
    items.first().cloned()
}
