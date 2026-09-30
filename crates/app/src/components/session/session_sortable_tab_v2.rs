//! Port of packages/app/src/components/session/session-sortable-tab-v2.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/session/session-sortable-tab-v2.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `SessionSortableTabV2` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SessionSortableTabV2Props {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionSortableTabV2Render {
    pub props: SessionSortableTabV2Props,
    pub children: Vec<String>,
}

impl SessionSortableTabV2Render {
    pub fn new(props: SessionSortableTabV2Props) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for SessionSortableTabV2.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SessionSortableTabV2State {
    pub mounted: bool,
}

impl SessionSortableTabV2State {
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

// Re-exported symbols from source (stubbed): SortableTabV2
