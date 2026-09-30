//! Port of packages/app/src/components/dialog-command-palette-v2.css
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// CSS passthrough for `packages/app/src/components/dialog-command-palette-v2.css` — no logic, validated at build.
pub const CSS: &str = include_str!("dialog-command-palette-v2.css");
/// Descriptor mirroring CSS asset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CssDescriptor {
    pub path: &'static str,
    pub content: &'static str,
}
pub const DESCRIPTOR: CssDescriptor = CssDescriptor {
    path: "packages/app/src/components/dialog-command-palette-v2.css",
    content: "",
};
