#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals, non_camel_case_types)] // 1:1 TS names (STALE_MS, RUN_ENTRY_NONE, provider IDs) stay verbatim
#![allow(clippy::module_inception)] // mirrors source dirs tool/mod.rs, v1/config/mod.rs, flag/mod.rs, id/mod.rs 1:1

//! Rust port of `@opencode-ai/core` v1.18.30.
//!
//! Source pin: commit 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings/
//! keys/defaults/ordering. Source is spec (`packages/core`).
//!
//! DOCTRINE: 1:1 exact translation — no improvements, renames, merges, splits, or reordering.
//!
//! PROVISIONAL inventory: imports from unported crates (`crates/database`, `crates/effect`,
//! `crates/filesystem`, native bindings for ripgrep/fff/pty) are flagged
//! `// PROVISIONAL pending <crate>` per module. See report for full inventory.
//! Behavior strings/IDs/defaults are copied verbatim from source; wiring is stubbed
//! until upstream crates land. No silent reinterpretation.

// Root modules (mirrors packages/core/src/*.ts — source order preserved)
pub mod account;
pub mod agent;
pub mod aisdk;
pub mod background_job;
pub mod catalog;
pub mod command;
pub mod config;
pub mod config_ts;
pub mod control_plane;
pub mod credential;
pub mod cross_spawn_spawner;
pub mod data_migration_sql;
pub mod database;
pub mod event;
pub mod file;
pub mod file_mutation;
pub mod filesystem;
pub mod fs_util;
pub mod git;
pub mod github_copilot;
pub mod global;
pub mod image;
pub mod instruction_context;
pub mod integration;
pub mod location;
pub mod location_mutation;
pub mod location_service_map;
pub mod location_services;
pub mod model;
pub mod models_dev;
pub mod npm;
pub mod npm_config;
pub mod observability;
pub mod patch;
pub mod permission;
pub mod plugin;
pub mod policy;
pub mod process;
pub mod project;
pub mod provider;
pub mod pty;
pub mod public_event_manifest;
pub mod question;
pub mod reference;
pub mod repository;
pub mod repository_cache;
pub mod ripgrep;
pub mod schema;
pub mod session;
pub mod shell;
pub mod skill;
pub mod snapshot;
pub mod state;
pub mod tool;
pub mod tool_output_store;
pub mod v1;
pub mod v2_schema;
pub mod workspace;

// Singleton subdirs (dash → underscore)
pub mod flag;
pub mod id;
pub mod installation;
pub mod oauth;
pub mod share;
pub mod system_context;

// Barrel re-exports in source order (mirrors TS exports):
pub use aisdk::*;
pub use command::*;
pub use control_plane::*;
pub use file::*;
pub use git::*;
pub use github_copilot::*;
pub use image::*;
pub use integration::*;
pub use location::*;
pub use models_dev::*;
pub use observability::*;
// NOTE: permission::* and session::* both contain a `sql` submodule (mirrors source
// permission/sql.ts + session/sql.ts 1:1); both stay reachable via full paths
// core::permission::sql and core::session::sql. First-glob-wins for bare `sql`.
#[allow(ambiguous_glob_reexports)]
pub use permission::*;
pub use plugin::*;
pub use process::*;
pub use project::*;
pub use provider::*;
pub use question::*;
pub use reference::*;
pub use ripgrep::*;
#[allow(ambiguous_glob_reexports)]
// see permission::* note above: session::sql reachable via full path
pub use session::*;
pub use shell::*;
pub use skill::*;
pub use snapshot::*;
pub use tool::*;
