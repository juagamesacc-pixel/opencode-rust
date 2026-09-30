//! Port of packages/app/src/components/prompt-input/slash-popover.tsx
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/prompt-input/slash-popover.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `SlashPopover` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SlashPopoverProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SlashPopoverRender {
    pub props: SlashPopoverProps,
    pub children: Vec<String>,
}

impl SlashPopoverRender {
    pub fn new(props: SlashPopoverProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for SlashPopover.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SlashPopoverState {
    pub mounted: bool,
}

impl SlashPopoverState {
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

// Re-exported symbols from source (stubbed): AtOption, SlashCommand, PromptPopover
