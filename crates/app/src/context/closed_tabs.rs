//! Rust port of `packages/app/src/context/closed-tabs.ts` (opencode v1.18.30).
//!
//! Source 40 lines. Exports: `ClosedTab`, `pushClosedTab`, `takeClosedTab`, `removeClosedTabs`, `nextTabAfterClose`.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

pub const CLOSED_TAB_LIMIT: usize = 25;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionTab {
    pub server: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "type")]
    pub tab_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DraftTab {
    #[serde(rename = "type")]
    pub tab_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Tab {
    #[serde(rename = "session")]
    Session(SessionTab),
    #[serde(rename = "draft")]
    Draft { id: String },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedTab {
    pub tab: SessionTab,
    pub index: usize,
}

#[allow(non_snake_case)]
pub fn pushClosedTab(stack: Vec<ClosedTab>, tab: &Tab, index: usize) -> Vec<ClosedTab> {
    match tab {
        Tab::Session(s) => {
            let mut next = stack;
            next.push(ClosedTab {
                tab: s.clone(),
                index,
            });
            if next.len() > CLOSED_TAB_LIMIT {
                let skip = next.len() - CLOSED_TAB_LIMIT;
                next = next.into_iter().skip(skip).collect();
            }
            next
        }
        _ => stack,
    }
}

#[allow(non_snake_case)]
pub fn takeClosedTab(
    mut stack: Vec<ClosedTab>,
    tabs: &[Tab],
) -> (Option<ClosedTab>, Vec<ClosedTab>) {
    while let Some(entry) = stack.pop() {
        if !isOpen(tabs, &entry.tab) {
            return (Some(entry), stack);
        }
    }
    (None, stack)
}

#[allow(non_snake_case)]
pub fn removeClosedTabs(
    stack: Vec<ClosedTab>,
    server: &str,
    session_ids: &[String],
) -> Vec<ClosedTab> {
    let removed: std::collections::HashSet<&String> = session_ids.iter().collect();
    stack
        .into_iter()
        .filter(|entry| entry.tab.server != server || !removed.contains(&entry.tab.session_id))
        .collect()
}

#[allow(non_snake_case)]
pub fn nextTabAfterClose(tabs: &[Tab], index: usize, active: bool) -> Option<Option<Tab>> {
    // Mirrors TS: if (!active) return undefined; return tabs[index+1] ?? tabs[index-1] ?? null
    if !active {
        return None;
    }
    let next = tabs.get(index + 1).cloned().or_else(|| {
        if index > 0 {
            tabs.get(index - 1).cloned()
        } else {
            None
        }
    });
    Some(next)
}

fn isOpen(tabs: &[Tab], tab: &SessionTab) -> bool {
    tabs.iter().any(|item| match item {
        Tab::Session(s) => s.server == tab.server && s.session_id == tab.session_id,
        _ => false,
    })
}
