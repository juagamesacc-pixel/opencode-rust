//! Rust port of `packages/desktop/src` (opencode v1.18.30).
//!
//! 1:1 translation of the Electron desktop app: main process, preload bridge,
//! IPC channels, application menu, updater, WSL servers, and renderer helpers.
//! Electron/Node/Solid runtime bindings are PROVISIONAL stubs mirroring the
//! source API surface; pure logic and app state are fully ported.
//!
//! See `/root/opencode-rust/plan-for-desktop.md` for the artifact inventory,
//! the 1:1 TS→Rust mapping, and the PROVISIONAL registry.

pub mod main;
pub mod preload;
pub mod renderer;
