//! Utils — mirrors `packages/app/src/utils/*` (opencode v1.18.30).
//!
//! Rename log (hyphen → underscore): `comment-note.ts` → `comment_note`,
//! `draft-store.ts` → `draft_store`, `file-manager.ts` → `file_manager`,
//! `menu-dismiss-controller.ts` → `menu_dismiss_controller`, `path-key.ts` → `path_key`,
//! `runtime-adapters.ts` → `runtime_adapters`, `scoped-cache.ts` → `scoped_cache`,
//! `search-keydown.ts` → `search_keydown`, `server-compat.ts` → `server_compat`,
//! `server-errors.ts` → `server_errors`, `server-health.ts` → `server_health`,
//! `server-protocol.ts` → `server_protocol`, `server-scope.ts` → `server_scope`,
//! `session-export.ts` → `session_export`, `session-message.ts` → `session_message`,
//! `session-route.ts` → `session_route`, `session-title.ts` → `session_title`,
//! `solid-dnd.tsx` → `solid_dnd`, `terminal-websocket-url.ts` → `terminal_websocket_url`,
//! `terminal-writer.ts` → `terminal_writer`, `toast.tsx` → `toast`.

#![allow(dead_code)]

pub mod agent;
pub mod aim;
pub mod base64;
pub mod comment_note;
pub mod diffs;
pub mod draft_store;
pub mod file_manager;
pub mod id;
pub mod menu_dismiss_controller;
pub mod path_key;
pub mod persist;
pub mod prompt;
pub mod refcount;
pub mod runtime_adapters;
pub mod same;
pub mod scoped_cache;
pub mod search_keydown;
pub mod server;
pub mod server_compat;
pub mod server_errors;
pub mod server_health;
pub mod server_protocol;
pub mod server_scope;
pub mod session;
pub mod session_export;
pub mod session_message;
pub mod session_route;
pub mod session_title;
pub mod solid_dnd;
pub mod sound;
pub mod terminal_websocket_url;
pub mod terminal_writer;
pub mod time;
pub mod toast;
pub mod uuid;
pub mod worktree;
