//! Port of packages/app/src/components/settings-v2/settings-v2.css
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `settings-v2` → `settings_v2` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// CSS passthrough for `packages/app/src/components/settings-v2/settings-v2.css` — no logic, validated at build.
pub const CSS: &str = include_str!("settings-v2.css");
/// Descriptor mirroring CSS asset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CssDescriptor {
    pub path: &'static str,
    pub content: &'static str,
}
pub const DESCRIPTOR: CssDescriptor = CssDescriptor {
    path: "packages/app/src/components/settings-v2/settings-v2.css",
    content: "",
};
