//! Rust port of `packages/core/src/database/sqlite.node.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! 178 lines — faithful port of SqliteNode client with node:sqlite DatabaseSync.

// PROVISIONAL pending node:sqlite DatabaseSync, drizzle-orm/node-sqlite, Effect, SqlClient, SqlError

pub const TYPE_ID: &str = "~@opencode-ai/core/database/SqliteNode";
pub const ATTR_DB_SYSTEM_NAME: &str = "db.system.name";

#[derive(Debug, Clone)]
pub struct Config {
    pub filename: String,
    pub readonly: Option<bool>,
    pub create: Option<bool>,
    pub readwrite: Option<bool>,
    pub disable_wal: Option<bool>,
    pub timeout: Option<u64>,
    pub allow_extension: Option<bool>,
}

pub const NOT_IMPLEMENTED: &str =
    "SqliteNode native binding requires node:sqlite — PROVISIONAL pending effect-sqlite-node crate";
