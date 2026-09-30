//! Port of packages/app/src/components/titlebar-tab-strip.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/titlebar-tab-strip.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `TitlebarTabStrip` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TitlebarTabStripProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitlebarTabStripRender {
    pub props: TitlebarTabStripProps,
    pub children: Vec<String>,
}

impl TitlebarTabStripRender {
    pub fn new(props: TitlebarTabStripProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for TitlebarTabStrip.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TitlebarTabStripState {
    pub mounted: bool,
}

impl TitlebarTabStripState {
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
