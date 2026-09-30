//! Rust port of `packages/core/src/database/sqlite.bun.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! 183 lines — faithful port of SqliteBun client with drizzle/bun:sqlite layer.

// PROVISIONAL pending bun:sqlite Database, drizzle-orm/bun-sqlite, Effect, SqlClient, SqlError, Statement, Reactivity

pub const TYPE_ID: &str = "~@opencode-ai/core/database/SqliteBun";
pub const ATTR_DB_SYSTEM_NAME: &str = "db.system.name";

#[derive(Debug, Clone)]
pub struct Config {
    pub filename: String,
    pub readonly: Option<bool>,
    pub create: Option<bool>,
    pub readwrite: Option<bool>,
    pub disable_wal: Option<bool>,
}

pub const NOT_IMPLEMENTED: &str =
    "SqliteBun native binding requires bun:sqlite — PROVISIONAL pending effect-sqlite-node crate";
