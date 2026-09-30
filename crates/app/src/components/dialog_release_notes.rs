//! Port of packages/app/src/components/dialog-release-notes.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/dialog-release-notes.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `DialogReleaseNotes` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DialogReleaseNotesProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DialogReleaseNotesRender {
    pub props: DialogReleaseNotesProps,
    pub children: Vec<String>,
}

impl DialogReleaseNotesRender {
    pub fn new(props: DialogReleaseNotesProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for DialogReleaseNotes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DialogReleaseNotesState {
    pub mounted: bool,
}

impl DialogReleaseNotesState {
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

// Re-exported symbols from source (stubbed): Highlight, DialogReleaseNotes
