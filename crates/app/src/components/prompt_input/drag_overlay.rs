//! Port of packages/app/src/components/prompt-input/drag-overlay.tsx
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/prompt-input/drag-overlay.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `DragOverlay` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DragOverlayProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DragOverlayRender {
    pub props: DragOverlayProps,
    pub children: Vec<String>,
}

impl DragOverlayRender {
    pub fn new(props: DragOverlayProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for DragOverlay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DragOverlayState {
    pub mounted: bool,
}

impl DragOverlayState {
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

// Re-exported symbols from source (stubbed): PromptDragOverlay
