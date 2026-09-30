//! Rust port of `packages/app/src/utils/file-manager.ts` (opencode v1.18.30).
//!
//! Source 24 lines: `FileManagerOS`, `fileManagerApp` (verbatim i18n keys).
//! Original file: `packages/app/src/utils/file-manager.ts`

#![allow(dead_code)]

/// Mirrors `FileManagerOS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileManagerOs {
    Macos,
    Windows,
    Linux,
    Unknown,
}

/// Mirrors the `fileManagerApp` return shape (verbatim i18n keys).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileManagerApp {
    pub label: &'static str,
    pub action_label: &'static str,
    pub icon: &'static str,
}

/// Mirrors `fileManagerApp(os)`.
pub fn file_manager_app(os: FileManagerOs) -> FileManagerApp {
    match os {
        FileManagerOs::Macos => FileManagerApp {
            label: "session.header.open.finder",
            action_label: "session.header.reveal.finder",
            icon: "finder",
        },
        FileManagerOs::Windows => FileManagerApp {
            label: "session.header.open.fileExplorer",
            action_label: "session.header.reveal.fileExplorer",
            icon: "file-explorer",
        },
        _ => FileManagerApp {
            label: "session.header.open.fileManager",
            action_label: "session.header.reveal.containingFolder",
            icon: "finder",
        },
    }
}
