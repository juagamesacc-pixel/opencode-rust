//! Port of packages/app/src/components/settings-v2/general-controller-behavior.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `settings-v2` → `settings_v2`.
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShellOption {
    pub path: String,
    pub name: String,
    pub acceptable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShellSelectOption {
    pub id: String,
    pub value: String,
    pub name: String,
    pub terminal_only: bool,
}

pub fn create_shell_options(input: &ShellOptionsInput) -> Vec<ShellSelectOption> {
    let mut counts = std::collections::HashMap::new();
    for s in &input.shells {
        *counts.entry(s.name.clone()).or_insert(0) += 1;
    }
    let mut options = vec![ShellSelectOption {
        id: "auto".to_string(),
        value: "".to_string(),
        name: "".to_string(),
        terminal_only: false,
    }];
    for shell in &input.shells {
        let ambiguous = counts.get(&shell.name).copied().unwrap_or(0) > 1;
        let name = if ambiguous {
            shell.path.clone()
        } else {
            shell.name.clone()
        };
        options.push(ShellSelectOption {
            id: shell.path.clone(),
            value: if ambiguous {
                shell.path.clone()
            } else {
                shell.name.clone()
            },
            name,
            terminal_only: !shell.acceptable,
        });
    }
    if let Some(cur) = &input.current {
        if !options.iter().any(|o| &o.value == cur) {
            options.push(ShellSelectOption {
                id: cur.clone(),
                value: cur.clone(),
                name: cur.clone(),
                terminal_only: false,
            });
        }
    }
    options
}

pub struct ShellOptionsInput {
    pub shells: Vec<ShellOption>,
    pub current: Option<String>,
}
