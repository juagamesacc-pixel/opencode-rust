// source: packages/tui/src/context/clipboard.tsx (18 lines, v1.18.30)
// 1:1 port — the context value is an explicit struct defaulting to the
// real `crate::clipboard` backend; overrides replace the whole service.

#![allow(dead_code)]

use std::sync::Arc;

/// Clipboard writer seam (`ClipboardService.write?`).
pub type ClipboardWriter = Arc<dyn Fn(&str) + Send + Sync>;
/// Clipboard reader seam (`ClipboardService.read?`).
pub type ClipboardReader = Arc<dyn Fn() -> Option<ClipboardContent> + Send + Sync>;

use crate::clipboard;

/// Mirrors `ClipboardContent`.
#[derive(Debug, Clone, Default)]
pub struct ClipboardContent {
    pub data: String,
    pub mime: String,
}

/// Mirrors `ClipboardService` (`read?`/`write?` stay optional).
#[derive(Clone, Default)]
pub struct ClipboardService {
    pub read: Option<ClipboardReader>,
    pub write: Option<ClipboardWriter>,
}

impl ClipboardService {
    /// Default value (mirrors `createContext<ClipboardService>(clipboard)`).
    pub fn system() -> Self {
        Self {
            read: Some(Arc::new(|| {
                crate::clipboard::read().map(|content| ClipboardContent {
                    data: content.data,
                    mime: content.mime,
                })
            })),
            write: Some(Arc::new(clipboard::write)),
        }
    }

    pub fn read(&self) -> Option<ClipboardContent> {
        self.read.as_ref().and_then(|read| read())
    }

    pub fn write(&self, text: &str) {
        if let Some(write) = self.write.as_ref() {
            write(text);
        }
    }
}
