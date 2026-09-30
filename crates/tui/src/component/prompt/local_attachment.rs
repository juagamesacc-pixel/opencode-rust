// source: packages/tui/src/component/prompt/local-attachment.ts (48 lines, v1.18.30)
// 1:1 port — mime table verbatim; svg reads as text, images/pdf as
// bytes; anything else (or any failure) yields `None`.

#![allow(dead_code)]

use std::path::Path;

/// Mime table verbatim.
pub const MIME_TYPES: &[(&str, &str)] = &[
    (".avif", "image/avif"),
    (".gif", "image/gif"),
    (".jpeg", "image/jpeg"),
    (".jpg", "image/jpeg"),
    (".pdf", "application/pdf"),
    (".png", "image/png"),
    (".svg", "image/svg+xml"),
    (".webp", "image/webp"),
];
const DEFAULT_MIME: &str = "application/octet-stream";

/// File reader seam (mirrors `LocalFiles`).
pub trait LocalFiles {
    fn read_text(&self, path: &str) -> Option<String>;
    fn read_bytes(&self, path: &str) -> Option<Vec<u8>>;
    fn mime(&self, path: &str) -> Option<String>;
}

/// Filesystem implementation.
pub struct FsLocalFiles;

impl LocalFiles for FsLocalFiles {
    fn read_text(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    fn read_bytes(&self, path: &str) -> Option<Vec<u8>> {
        std::fs::read(path).ok()
    }

    fn mime(&self, path: &str) -> Option<String> {
        Some(
            Path::new(path)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| {
                    let key = format!(".{}", ext.to_lowercase());
                    MIME_TYPES
                        .iter()
                        .find(|(extension, _)| *extension == key)
                        .map(|(_, mime)| mime.to_string())
                })
                .flatten()
                .unwrap_or_else(|| DEFAULT_MIME.to_string()),
        )
    }
}

/// Mirrors `LocalAttachment`.
#[derive(Debug, Clone)]
pub enum LocalAttachment {
    Text { mime: String, content: String },
    Binary { mime: String, content: Vec<u8> },
}

/// Mirrors `readLocalAttachmentWith`.
pub fn read_local_attachment_with(files: &dyn LocalFiles, path: &str) -> Option<LocalAttachment> {
    let mime = files.mime(path)?;
    if mime == "image/svg+xml" {
        let content = files.read_text(path)?;
        if content.is_empty() {
            return None;
        }
        return Some(LocalAttachment::Text { mime, content });
    }
    if !mime.starts_with("image/") && mime != "application/pdf" {
        return None;
    }
    let content = files.read_bytes(path)?;
    if content.is_empty() {
        return None;
    }
    Some(LocalAttachment::Binary { mime, content })
}

/// Mirrors `readLocalAttachment` (filesystem backend).
pub fn read_local_attachment(path: &str) -> Option<LocalAttachment> {
    read_local_attachment_with(&FsLocalFiles, path)
}
