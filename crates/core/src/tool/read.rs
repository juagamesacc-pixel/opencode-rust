//! Rust port of `packages/core/src/tool/read.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "read";

pub const SUPPORTED_IMAGE_MIMES: &[&str] = &["image/jpeg", "image/png", "image/gif", "image/webp"];

pub fn is_supported_image_mime(mime: &str) -> bool {
    SUPPORTED_IMAGE_MIMES.contains(&mime)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInput {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

pub type Input = LocationInput;

// Output is union of FileSystem.Content | TextPage | ListPage — keep as serde_json::Value for 1:1
pub type Output = serde_json::Value;

pub fn to_model_output_for_image(path: &str, mime: &str, content: &str) -> Vec<serde_json::Value> {
    if !is_supported_image_mime(mime) {
        return vec![];
    }
    vec![
        serde_json::json!({"type": "text", "text": "Image read successfully"}),
        serde_json::json!({"type": "file", "data": content, "mime": mime, "name": path}),
    ]
}

// PROVISIONAL pending effect/runtime — Layer wiring requires ReadToolFileSystem + LocationMutation + Image + PermissionV2.
