// source: src/index.ts — exports: EffectLogger (re-export), * from effect-sqlite/driver, * from effect-sqlite/session, migrate, EffectDrizzleSqlite namespace
//! 1:1 port of `@opencode-ai/effect-drizzle-sqlite` index barrel.
//! Source pin: v1.18.30 @3104c14.

#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod effect_sqlite;
pub mod internal;
pub mod sqlite_core;
pub mod up_migrations;

// Re-exports mirroring `export { EffectLogger } from "drizzle-orm/effect-core"` — PROVISIONAL pending drizzle-orm/effect-core (no Rust equivalent yet)
pub mod effect_logger {
    // PROVISIONAL pending drizzle-orm/effect-core — minimal descriptor preserving service ID
    pub const SERVICE_ID: &str = "EffectLogger";
}

// Mirroring `export * from "./effect-sqlite/driver"` and `export * from "./effect-sqlite/session"`
pub use effect_sqlite::driver::{
    make, make_with_defaults, DefaultServices, EffectDrizzleSQLiteConfig, EffectSQLiteDatabase,
};
pub use effect_sqlite::session::{
    EffectSQLiteQueryEffectHKT, EffectSQLiteRunResult, EffectSQLiteSession, EffectSQLiteTransaction,
};

// Mirroring `export { migrate } from "./effect-sqlite/migrator"`
pub use effect_sqlite::migrator::migrate;

// Self-namespace `export * as EffectDrizzleSqlite from "."` — barrel re-export
pub mod EffectDrizzleSqlite {
    pub use super::effect_sqlite::driver::EffectSQLiteDatabase;
    pub use super::effect_sqlite::migrator::migrate;
}
