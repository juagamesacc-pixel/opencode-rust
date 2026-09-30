// source: src/config/*.ts (barrel — self-reexport pattern per AGENTS.md;
// no index.ts in source; module list mirrors directory lexical order).
pub mod agent;
pub mod command;
pub mod config;
pub mod entry_name;
pub mod managed;
pub mod markdown;
pub mod parse;
pub mod paths;
pub mod plugin;
pub mod tui;
pub mod tui_cwd;
pub mod tui_host_attention;
pub mod tui_migrate;
pub mod v2_compat;
pub mod variable;
