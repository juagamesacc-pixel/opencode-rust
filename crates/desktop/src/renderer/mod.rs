//! Rust port of `packages/desktop/src/renderer/` (opencode v1.18.30).
//!
//! Ambient declarations from `src/renderer/env.d.ts` (no runtime code in source):
//! - `window.api: ElectronAPI` → [`crate::preload::types::ElectronApi`];
//!   the concrete bridge object is built in [`crate::preload::index`].
//! - `window.__OPENCODE__?: { deepLinks?: string[] }` →
//!   [`crate::renderer::index::OpencodeWindowState`] (`deep_links: Vec<String>`).

pub mod cli;
pub mod i18n;
pub mod index;
pub mod initialization;
pub mod onboarding;
pub mod webview_zoom;
pub mod window_fullscreen;
pub mod wsl;
