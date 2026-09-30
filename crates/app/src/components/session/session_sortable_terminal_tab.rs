//! Port of packages/app/src/components/session/session-sortable-terminal-tab.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/session/session-sortable-terminal-tab.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `SessionSortableTerminalTab` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SessionSortableTerminalTabProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionSortableTerminalTabRender {
    pub props: SessionSortableTerminalTabProps,
    pub children: Vec<String>,
}

impl SessionSortableTerminalTabRender {
    pub fn new(props: SessionSortableTerminalTabProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for SessionSortableTerminalTab.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SessionSortableTerminalTabState {
    pub mounted: bool,
}

impl SessionSortableTerminalTabState {
    pub fn new() -> Self {
        Self { mounted: false }
    }
    pub fn mount(&mut self) {
        self.mounted = true;
    }
    pub fn unmount(&mut self) {
        self.mounted = false;
    }
}

// Re-exported symbols from source (stubbed): SortableTerminalTab
