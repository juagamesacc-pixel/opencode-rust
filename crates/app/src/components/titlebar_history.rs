//! Port of packages/app/src/components/titlebar-history.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub const MAX_TITLEBAR_HISTORY: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitlebarHistory {
    pub stack: Vec<String>,
    pub index: usize,
    pub action: Option<String>,
}

pub fn apply_path(state: TitlebarHistory, current: String, max: usize) -> TitlebarHistory {
    if state.stack.is_empty() {
        let stack = if current == "/" {
            vec!["/".to_string()]
        } else {
            vec!["/".to_string(), current]
        };
        let index = stack.len() - 1;
        return TitlebarHistory {
            stack,
            index,
            action: None,
        };
    }
    let active = state.stack.get(state.index).cloned().unwrap_or_default();
    if current == active {
        if state.action.is_none() {
            return state;
        }
        return TitlebarHistory {
            action: None,
            ..state
        };
    }
    if state.action.is_some() {
        return TitlebarHistory {
            action: None,
            ..state
        };
    }
    push_path(state, current, max)
}

pub fn push_path(state: TitlebarHistory, path: String, max: usize) -> TitlebarHistory {
    let mut stack = state.stack[..=state.index].to_vec();
    stack.push(path);
    let trimmed = trim_history(stack.clone(), stack.len() - 1, max);
    TitlebarHistory {
        stack: trimmed.stack,
        index: trimmed.index,
        action: None,
    }
}

pub struct TrimResult {
    pub stack: Vec<String>,
    pub index: usize,
}

pub fn trim_history(stack: Vec<String>, index: usize, max: usize) -> TrimResult {
    if stack.len() <= max {
        return TrimResult { stack, index };
    }
    let cut = stack.len() - max;
    TrimResult {
        stack: stack[cut..].to_vec(),
        index: index.saturating_sub(cut),
    }
}

pub struct NavResult {
    pub state: TitlebarHistory,
    pub to: String,
}

pub fn back_path(state: TitlebarHistory) -> Option<NavResult> {
    if state.index == 0 {
        return None;
    }
    let index = state.index - 1;
    let to = state.stack.get(index)?.clone();
    Some(NavResult {
        state: TitlebarHistory {
            index,
            action: Some("back".to_string()),
            ..state
        },
        to,
    })
}

pub fn forward_path(state: TitlebarHistory) -> Option<NavResult> {
    if state.index + 1 >= state.stack.len() {
        return None;
    }
    let index = state.index + 1;
    let to = state.stack.get(index)?.clone();
    Some(NavResult {
        state: TitlebarHistory {
            index,
            action: Some("forward".to_string()),
            ..state
        },
        to,
    })
}
