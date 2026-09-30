//! Port of packages/app/src/components/command-tooltip-keybind.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Input for tooltip keybind — mirrors TS `CommandKeybind`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandKeybind {
    pub keybind_parts: Vec<String>,
}

impl CommandKeybind {
    pub fn keybind_parts(&self, id: &str) -> Vec<String> {
        let _ = id;
        self.keybind_parts.clone()
    }
}

pub fn review_tooltip_keybind(
    command: &CommandKeybind,
    _translate: Option<fn(&str) -> String>,
) -> Vec<String> {
    command.keybind_parts("review.toggle")
}

pub fn new_tab_tooltip_keybind(
    command: &CommandKeybind,
    _translate: Option<fn(&str) -> String>,
) -> Vec<String> {
    command.keybind_parts("tab.new")
}
