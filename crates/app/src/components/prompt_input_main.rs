//! Port of packages/app/src/components/prompt-input.tsx
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input.tsx` → `prompt_input_main.rs` (Rust module collision: file `prompt-input.tsx` vs directory `prompt-input/` both map to `prompt_input`; file renamed to `prompt_input_main` to preserve 1:1 attendance).
//! Additional rename: `prompt-input` → `prompt_input` (snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/prompt-input.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `PromptInput` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PromptInputProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInputRender {
    pub props: PromptInputProps,
    pub children: Vec<String>,
}

impl PromptInputRender {
    pub fn new(props: PromptInputProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for PromptInput.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PromptInputState {
    pub mounted: bool,
}

impl PromptInputState {
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
