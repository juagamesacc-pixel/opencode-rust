// source: src/sqlite-core/effect/session.ts — exports: SQLiteEffectSession, SQLiteEffectTransaction, SQLiteEffectPreparedQuery, migrate helper
//! Core drizzle Effect session wiring — transaction/savepoint logic preserved via effect_sqlite::session
//! PROVISIONAL pending Effect SqlClient + drizzle internal session runtime

#![allow(dead_code)]
pub struct SQLiteEffectSession;
pub struct SQLiteEffectTransaction;
pub struct SQLiteEffectPreparedQuery;

pub fn migrate(
    _migrations: serde_json::Value,
    _session: &SQLiteEffectSession,
    _config: serde_json::Value,
) -> Result<(), String> {
    // PROVISIONAL pending drizzle-orm migrator file I/O
    Ok(())
}
