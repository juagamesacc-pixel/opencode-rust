#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

//! Rust port of @opencode-ai/tui v1.18.30 — exact 1:1 clone.
//! Source pin: v1.18.30 @3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2, OpenTUI + SolidJS.
//! Doctrine: source is spec — every screen/component/layout/color/spacing/keybinding/interaction/wording identical.
//! Solid reactivity → explicit Rust state machines (new()/update()/transition, same state keys/order);
//! OpenTUI widgets → ratatui with identical layout constraints; keymap → verbatim tables;
//! clipboard/open/diff/fuzzysort/strip-ansi → std or approved crates only (PROVISIONAL stubs flagged).

// Top-level modules (source order, snake_case)
pub mod app;
pub mod attention;
pub mod audio;
pub mod audio_d_ts;
pub mod clipboard;
pub mod component;
pub mod config;
pub mod context;
pub mod editor;
pub mod editor_zed;
pub mod feature_plugins;
pub mod keymap;
pub mod logo;
pub mod parsers_config;
pub mod plugin;
pub mod prompt;
pub mod routes;
pub mod runtime;
pub mod terminal_win32;
pub mod theme;
pub mod ui;
pub mod util;

// Re-exports (mirror src/index.tsx: `export { run, type TuiInput } from "./app"`)
pub use app::{run, TuiInput};
