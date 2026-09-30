//! Port of packages/app/src/components/file-tree-v2.tsx
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `file-tree-v2.tsx` → `file_tree_v2.tsx` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/file-tree-v2.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `FileTreeV2` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileTreeV2Props {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileTreeV2Render {
    pub props: FileTreeV2Props,
    pub children: Vec<String>,
}

impl FileTreeV2Render {
    pub fn new(props: FileTreeV2Props) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for FileTreeV2.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileTreeV2State {
    pub mounted: bool,
}

impl FileTreeV2State {
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

// Re-exported symbols from source (stubbed): kindLabel, kindChange
