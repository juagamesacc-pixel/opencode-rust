//! Rust port of `packages/core/src/image.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    pub content: String,
    pub encoding: String,
    pub mime: String,
}

#[derive(Debug)]
pub struct DecodeError {
    pub message: String,
}
#[derive(Debug)]
pub struct SizeError {
    pub message: String,
}
#[derive(Debug)]
pub struct ResizerUnavailableError;

pub fn is_supported_mime(mime: &str) -> bool {
    matches!(
        mime,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp"
    )
}

// PROVISIONAL pending image resizer native.
