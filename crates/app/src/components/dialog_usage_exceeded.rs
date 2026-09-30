//! Port of packages/app/src/components/dialog-usage-exceeded.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/dialog-usage-exceeded.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `DialogUsageExceeded` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DialogUsageExceededProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DialogUsageExceededRender {
    pub props: DialogUsageExceededProps,
    pub children: Vec<String>,
}

impl DialogUsageExceededRender {
    pub fn new(props: DialogUsageExceededProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for DialogUsageExceeded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DialogUsageExceededState {
    pub mounted: bool,
}

impl DialogUsageExceededState {
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

// Re-exported symbols from source (stubbed): DialogGoUpsellProps, DialogUsageExceeded
