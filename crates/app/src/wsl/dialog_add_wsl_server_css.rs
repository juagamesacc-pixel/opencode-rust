//! Port of `packages/app/src/wsl/dialog-add-wsl-server.css` — passthrough, no logic.
//! Original file: `packages/app/src/wsl/dialog-add-wsl-server.css`

#![allow(dead_code)]

/// CSS passthrough — validated at build.
pub const CSS: &str = include_str!("dialog-add-wsl-server.css");

/// Descriptor mirroring CSS asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssDescriptor {
    pub path: &'static str,
    pub content: &'static str,
}

pub const DESCRIPTOR: CssDescriptor = CssDescriptor {
    path: "packages/app/src/wsl/dialog-add-wsl-server.css",
    content: "",
};
