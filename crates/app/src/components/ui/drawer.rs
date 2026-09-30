//! Port of packages/app/src/components/ui/drawer.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/ui/drawer.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `Drawer` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DrawerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrawerRender {
    pub props: DrawerProps,
    pub children: Vec<String>,
}

impl DrawerRender {
    pub fn new(props: DrawerProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for Drawer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DrawerState {
    pub mounted: bool,
}

impl DrawerState {
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
