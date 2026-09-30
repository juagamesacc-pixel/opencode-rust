//! Context modules — mirrors packages/app/src/context/* (opencode v1.18.30).
//!
//! Rename log:
//! - global-sync → global_sync (Rust identifier hyphen → underscore)
//! - file.tsx → file_context.rs (collision with file/ directory — Rust module name conflict; original file.ts vs dir)
//! - All other file hyphen → underscore per snake_case.

#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod closed_tabs;
pub mod command;
pub mod comments;
pub mod directory_sync;
pub mod file;
pub mod file_context;
pub mod global;
pub mod global_sync;
pub mod highlights;
pub mod language;
pub mod layout;
pub mod layout_helpers;
pub mod layout_scroll;
pub mod layout_tabs;
pub mod local;
pub mod local_agent;
pub mod mcp;
pub mod model_variant;
pub mod models;
pub mod notification;
pub mod permission;
pub mod permission_auto_respond;
pub mod platform;
pub mod prompt;
pub mod prompt_state;
pub mod sdk;
pub mod server;
pub mod server_sdk;
pub mod server_session;
pub mod server_session_v2_reducer;
pub mod server_sync;
pub mod settings;
pub mod sync;
pub mod tab_memory;
pub mod tab_migration;
pub mod tabs;
pub mod terminal;
pub mod terminal_title;
