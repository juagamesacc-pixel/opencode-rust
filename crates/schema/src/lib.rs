#![allow(clippy::result_large_err)]
// large Err types mirror source Effect error channels; boxing 150 paths = churn with zero behavior gain
#![allow(non_upper_case_globals)] // source exports `Definitions`/`DurableDefinitions` exactly; renaming violates 1:1
#![allow(clippy::redundant_static_lifetimes)]
// `pub const Definitions: &[&str]` mirrors `export const Definitions` with implicit 'static; explicit 'static is noise
#![allow(clippy::large_enum_variant)] // large variants mirror source; boxing = churn
#![allow(dead_code)] // private helpers mirror source; never remove pub items
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)] // aliases added for the worst cases; remaining are source-mirroring
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]

//! Rust port of `@opencode-ai/schema` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/schema`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.

pub mod agent;
pub mod catalog;
pub mod command;
pub mod connection;
pub mod credential;
pub mod durable_event_manifest;
pub mod event;
pub mod event_manifest;
pub mod file_diff;
pub mod filesystem;
pub mod filesystem_watcher;
pub mod ide_event;
pub mod identifier;
pub mod installation_event;
pub mod integration;
pub mod integration_id;
pub mod legacy_event;
pub mod llm;
pub mod location;
pub mod lsp_event;
pub mod mcp_event;
pub mod model;
pub mod models_dev;
pub mod permission;
pub mod permission_saved;
pub mod permission_v1;
pub mod plugin;
pub mod project;
pub mod project_copy;
pub mod project_directories;
pub mod project_id;
pub mod prompt;
pub mod prompt_input;
pub mod provider;
pub mod pty;
pub mod pty_ticket;
pub mod question;
pub mod question_v1;
pub mod reference;
pub mod revert;
pub mod schema_primitives;
pub mod server_event;
pub mod session;
pub mod session_compaction_event;
pub mod session_delivery;
pub mod session_event;
pub mod session_id;
pub mod session_input;
pub mod session_message;
pub mod session_status_event;
pub mod session_todo;
pub mod session_v1;
pub mod skill;
pub mod tui_event;
pub mod v1;
pub mod vcs_event;
pub mod workspace;
pub mod workspace_event;
pub mod workspace_id;
pub mod worktree_event;
