//! Port of packages/app/src/components/prompt-input/image-attachments.css
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input` → `prompt_input` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// CSS passthrough for `packages/app/src/components/prompt-input/image-attachments.css` — no logic, validated at build.
pub const CSS: &str = include_str!("image-attachments.css");
/// Descriptor mirroring CSS asset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CssDescriptor {
    pub path: &'static str,
    pub content: &'static str,
}
pub const DESCRIPTOR: CssDescriptor = CssDescriptor {
    path: "packages/app/src/components/prompt-input/image-attachments.css",
    content: "",
};
