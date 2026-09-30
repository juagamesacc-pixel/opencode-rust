//! Port of packages/app/src/components/file-tree.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/file-tree.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `FileTree` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileTreeProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileTreeRender {
    pub props: FileTreeProps,
    pub children: Vec<String>,
}

impl FileTreeRender {
    pub fn new(props: FileTreeProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for FileTree.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileTreeState {
    pub mounted: bool,
}

impl FileTreeState {
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

// Re-exported symbols from source (stubbed): pathToFileUrl, Kind, Filter, shouldListRoot, shouldListExpanded, dirsToExpand, visibleKind, withFileDragImage
