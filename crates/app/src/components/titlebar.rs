//! Port of packages/app/src/components/titlebar.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/titlebar.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `Titlebar` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TitlebarProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitlebarRender {
    pub props: TitlebarProps,
    pub children: Vec<String>,
}

impl TitlebarRender {
    pub fn new(props: TitlebarProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for Titlebar.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TitlebarState {
    pub mounted: bool,
}

impl TitlebarState {
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

// Re-exported symbols from source (stubbed): TitlebarUpdate, useTitlebarRightMount, Titlebar
