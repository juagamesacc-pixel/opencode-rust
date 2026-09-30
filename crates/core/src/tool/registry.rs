//! Rust port of `packages/core/src/tool/registry.ts`.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExecuteInput {
    pub session_id: String,
    pub agent: String,
    pub assistant_message_id: String,
    pub call_name: String,
    pub call_id: String,
}

#[derive(Debug, Clone)]
pub struct Settlement {
    pub result_type: String,
    pub value: String,
}

pub fn wholly_disabled(action: &str, rules: &[(String, String, String)]) -> bool {
    // rules: (action_pattern, resource, effect) — mirrors PermissionV2.Ruleset
    // wildcard match via simple glob: "*" matches all, "prefix*" etc.
    let mut last: Option<&(String, String, String)> = None;
    for rule in rules {
        if wildcard_match(action, &rule.0) {
            last = Some(rule);
        }
    }
    if let Some((_, resource, effect)) = last {
        return resource == "*" && effect == "deny";
    }
    false
}

pub fn wildcard_match(input: &str, pattern: &str) -> bool {
    // Mirrors Wildcard.match — simple "*" and "prefix*" support
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return input.starts_with(prefix);
    }
    input == pattern
}

#[derive(Debug, Default)]
pub struct Registry {
    // local overlay: name -> stack of tokens; latest wins. ApplicationTools shared via separate map.
    local: HashMap<String, Vec<String>>,
    application: HashMap<String, String>,
}

impl Registry {
    pub fn register(&mut self, tools: HashMap<String, String>) -> Result<(), String> {
        for name in tools.keys() {
            crate::tool::tool::validate_name(name).map_err(|e| e.message)?;
        }
        for (name, _tool) in tools {
            self.local
                .entry(name)
                .or_default()
                .push("token".to_string());
        }
        Ok(())
    }

    pub fn materialize(&self, permissions: &[(String, String, String)]) -> Vec<String> {
        let mut regs = self.application.clone();
        for (name, stack) in &self.local {
            if let Some(token) = stack.last() {
                regs.insert(name.clone(), token.clone());
            }
        }
        regs.retain(|name, _| !wholly_disabled(name, permissions));
        regs.keys().cloned().collect()
    }
}

// PROVISIONAL pending ToolOutputStore + ApplicationTools service wiring + Effect Scope finalizers.
// Behavior for whollyDisabled, wildcard, overlay precedence preserved verbatim above.
